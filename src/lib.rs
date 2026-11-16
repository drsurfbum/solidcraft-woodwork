//! Woodworking plugin for SolidCraft. The modules are plain Rust that turn parameters into
//! app commands, so `cargo test` covers them; only `guest` touches the wasm bindings.

mod base;
mod cabinet;
mod corner;
mod cutlist;
mod fittings;
mod joinery;
mod kit;
mod material;
mod select;

use kit::check;
use serde_json::{json, Value};

/// App commands with their arguments, in the order they run.
pub type Commands = Vec<(&'static str, Value)>;

/// Runs one app command and returns its result.
pub type Run<'a> = &'a mut dyn FnMut(&str, Value) -> Result<Value, String>;

pub fn execute(entry: &str, params: &Value, run: Run) -> Result<Value, String> {
    match entry {
        "cabinet" | "corner" | "base" => {
            // Where it goes: on the origin, or against the cabinet that is selected.
            let place = params["place"].as_str().unwrap_or("origin");
            check(
                matches!(place, "origin" | "right" | "left" | "above"),
                "place must be origin, right, left or above",
            )?;
            let matching = match params["match"].as_str().unwrap_or("no") {
                "no" => false,
                "selected" => true,
                _ => return Err("match must be no or selected".into()),
            };
            let mut params = params.clone();
            let mut beside = None;
            let design = run(
                "document.inspect",
                json!({"bodies": false, "timeline": false}),
            )?;
            // A name that is taken gets the next free number: Cabinet, Cabinet 2, Cabinet 3.
            let name = params["name"].as_str().unwrap_or("Cabinet");
            params["name"] = json!(kit::free_name(name, &design));
            if matching || place != "origin" {
                let found = select::cabinet(&design, &run("inspect.measure", json!({}))?)?;
                if matching {
                    params["_match"] = kit::matched(&found.name, &design)?;
                }
                if place != "origin" {
                    beside = Some(found);
                }
            }
            let (commands, mut summary) = match entry {
                "cabinet" => cabinet::build(&params)?,
                "base" => base::build(&params)?,
                _ => corner::build(&params)?,
            };
            // The cabinet is a component of its own, inside whichever component is active:
            // it moves as one piece, and a room can hold its cabinets.
            let made = run(
                "component.create",
                json!({"name": summary["name"], "grounded": false}),
            )?;
            summary["occurrence"] = made["occurrence"].clone();
            let mut feature_ids = std::collections::BTreeMap::new();
            for (command, args) in commands {
                let feature_name = args["name"].as_str().map(String::from);
                let result = run(command, args)?;
                if let (Some(name), Some(id)) = (feature_name, result["feature"].as_u64()) {
                    feature_ids.insert(name, id);
                }
            }
            if entry == "base" {
                // Include patterned copies in the material assignment and returned part list.
                let measure = run("inspect.measure", json!({}))?;
                let prefix = format!("{} ", summary["name"].as_str().unwrap());
                summary["bodies"] = json!(measure["bodies"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter_map(|b| b["name"].as_str())
                    .filter(|name| name.starts_with(&prefix))
                    .collect::<Vec<_>>());
            }
            let board = params["material"].as_str().unwrap_or("Birch plywood");
            if board != "none" {
                let bodies = summary["bodies"].as_array().into_iter().flatten();
                let bodies: Vec<String> = bodies
                    .filter_map(|b| b.as_str())
                    .map(String::from)
                    .collect();
                material::assign(run, board, &bodies)?;
            }
            if entry == "base" {
                base::controls::install(run, &mut summary, &feature_ids)?;
                let measure = run("inspect.measure", json!({}))?;
                let prefix = format!("{} ", summary["name"].as_str().unwrap());
                summary["bodies"] = json!(measure["bodies"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter_map(|b| b["name"].as_str())
                    .filter(|n| n.starts_with(&prefix))
                    .collect::<Vec<_>>());
            }
            // Back to where the user was working.
            run("component.activate", json!({"component": made["parent"]}))?;
            if let Some(other) = beside {
                // Against that cabinet's side, or on top of it, turned the way it is turned.
                let size = |key: &str| summary["size"][key].as_f64().unwrap_or(0.0);
                let (at, degrees) = other.seat(place, size("width"), size("depth"));
                // plain numbers: this command takes mm whatever the design's unit is
                let mut to =
                    json!({"occurrence": made["occurrence"], "space": "world", "translate": at});
                if degrees != 0 {
                    to["axis"] = json!([0, 0, 1]);
                    to["angle"] = json!(format!("{degrees} deg"));
                }
                run("occurrence.move", to)?;
                summary["placed"] = json!(format!("{place} of {}", other.name));
            }
            // Grounded, so that its doors and drawers have something fixed to open against.
            // To move the cabinet later, unground it first.
            run(
                "occurrence.ground",
                json!({"occurrence": made["occurrence"], "grounded": true}),
            )?;
            Ok(summary)
        }
        "joinery" => {
            // Screw or dowel holes at the corners of the selected cabinet, drilled inside its
            // component.
            let design = run(
                "document.inspect",
                json!({"bodies": false, "timeline": "summary"}),
            )?;
            let found = select::cabinet(&design, &run("inspect.measure", json!({}))?)?;
            let kind = params["type"].as_str().unwrap_or("confirmat");
            let (commands, origin) = joinery::build(&found, &design, kind)?;
            let before = run("component.list", json!({"compact": true}))?["active"].clone();
            // In a plugin run the app shifts a hole's position by where the component stands,
            // so a cabinet that has been moved is drilled on the origin and then put back.
            // ponytail: this leaves the cabinet grounded, as the cabinet entry made it, also
            // if the user had freed it since.
            let occurrence = format!("{}:1", found.name);
            let shift = |by: [f64; 3], run: Run| {
                run(
                    "occurrence.move",
                    json!({"occurrence": occurrence, "translate": by}),
                )
            };
            let away = origin.iter().any(|v| v.abs() > 1e-6);
            if away {
                run(
                    "occurrence.ground",
                    json!({"occurrence": occurrence, "grounded": false}),
                )?;
                shift(origin.map(|v| -v), run)?;
            }
            run("component.activate", json!({"component": found.name}))?;
            for (command, args) in commands {
                run(command, args)?;
            }
            run("component.activate", json!({"component": before}))?;
            if away {
                shift(origin, run)?;
                run(
                    "occurrence.ground",
                    json!({"occurrence": occurrence, "grounded": true}),
                )?;
            }
            Ok(json!({"cabinet": found.name, "type": kind, "joints": 4}))
        }
        "material" => {
            // A board material for the selected cabinet, or for just the selected parts.
            let design = run(
                "document.inspect",
                json!({"bodies": false, "timeline": false}),
            )?;
            let bodies = match params["scope"].as_str().unwrap_or("cabinet") {
                "parts" => select::picked(&design),
                "cabinet" => {
                    let measure = run("inspect.measure", json!({}))?;
                    select::parts(&measure, &select::cabinet(&design, &measure)?.name)
                }
                _ => return Err("scope must be cabinet or parts".into()),
            };
            if bodies.is_empty() {
                return Err("select a cabinet or some of its parts first".into());
            }
            let board = params["material"].as_str().unwrap_or("Birch plywood");
            material::assign(run, board, &bodies)?;
            Ok(json!({"material": board, "bodies": bodies}))
        }
        "cutlist" => {
            // Inches when the design is in inches or feet, else mm.
            let design = run(
                "document.inspect",
                json!({"bodies": false, "timeline": "summary"}),
            )?;
            let imperial = matches!(design["units"].as_str(), Some("in" | "ft"));
            let measure = run("inspect.measure", json!({}))?;
            // ponytail: one more command per turned board, so a design with more than about
            // 250 boards standing at an angle runs into the 256-command limit.
            let mut local = std::collections::BTreeMap::new();
            for body in cutlist::turned(&measure) {
                let mesh = run("model.mesh", json!({"body": body, "tolerance": 1.0}))?;
                local.insert(body, cutlist::extents(&mesh));
            }
            let mut list = cutlist::cutlist(&measure, &local, imperial)?;
            list["hardware"] = cutlist::hardware(&measure, &design);
            Ok(list)
        }
        _ => Err(format!("unknown entry `{entry}`")),
    }
}

#[cfg(target_arch = "wasm32")]
mod guest {
    wit_bindgen::generate!({ path: "wit", world: "command-plugin" });

    use craft::plugin::types::{FieldFailure, PluginError};

    struct Woodwork;

    impl exports::craft::plugin::entry::Guest for Woodwork {
        fn execute(entry: String, params: String) -> Result<String, PluginError> {
            let invalid = |reason: String| {
                PluginError::InvalidInput(FieldFailure {
                    field: "params".into(),
                    reason,
                })
            };
            let params = serde_json::from_str(&params).map_err(|e| invalid(e.to_string()))?;
            let mut run = |command: &str, args: serde_json::Value| {
                let reply = craft::plugin::commands::run(command, &args.to_string()).map_err(
                    |e| match e {
                        PluginError::CommandFailed(f) => format!("{}: {}", f.command, f.reason),
                        other => format!("{command}: {other:?}"),
                    },
                )?;
                serde_json::from_str(&reply).map_err(|e| format!("{command}: bad reply: {e}"))
            };
            crate::execute(&entry, &params, &mut run)
                .map(|v| v.to_string())
                .map_err(invalid)
        }
    }

    export!(Woodwork);
}
