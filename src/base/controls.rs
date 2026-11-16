//! Reusable source controls. Configuration rules live in the saved vanilla design.
use crate::{
    kit::{check, num, Kit},
    Commands, Run,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub fn number(kit: &Kit, key: &str, value: f64, comment: &str) -> String {
    let name = format!("{}_{key}", kit.prefix);
    kit.push((
        "parameters.add",
        json!({"name":name,"expression":value.to_string(),"unit":"","comment":comment}),
    ));
    name
}

fn is(parameter: &str, operator: &str, expression: &str) -> Value {
    json!({"type":"parameter","parameter":parameter,"operator":operator,"expression":expression})
}

pub struct Controls {
    pub switches: Vec<Value>,
    pub parameters: Vec<String>,
    show: String,
    hardware: String,
    knobs: String,
    center: String,
    x: String,
    top: String,
    pull: String,
    projection: String,
}

impl Controls {
    pub fn new(kit: &Kit, p: &Value, drawers: bool) -> Result<Self, String> {
        let toggle = |key, default, comment| -> Result<String, String> {
            let v = match p[key].as_bool() {
                Some(x) => {
                    if x {
                        1.0
                    } else {
                        0.0
                    }
                }
                None => num(p, key, default)?,
            };
            check(
                v == 0.0 || v == 1.0,
                "visibility/centering choices must be 0 or 1",
            )?;
            Ok(number(kit, key, v, comment))
        };
        let show = toggle(
            "show_fronts",
            1.0,
            "1 shows fronts; 0 removes fronts, retaining the interior",
        )?;
        let hardware = toggle(
            "show_hardware",
            1.0,
            "1 shows hardware; 0 removes it; hidden fronts also hide hardware",
        )?;
        let knobs = toggle("use_knobs", 0.0, "1 uses round knobs; 0 uses bar pulls")?;
        let center = toggle(
            "center_hardware",
            1.0,
            "1 centers hardware vertically; 0 locates it by the top offset",
        )?;
        let parameters = vec![
            show.clone(),
            hardware.clone(),
            knobs.clone(),
            center.clone(),
        ];
        let x = kit.parameter(
            "hardware_x_offset",
            num(p, "hardware_x_offset", 0.0)?,
            if drawers {
                "horizontal shift from drawer center, positive right"
            } else {
                "shift inward from 38.1 mm at the door meeting edge"
            },
        );
        check(
            num(p, "hardware_projection", 25.4)? > 6.0,
            "hardware projection must exceed 6 mm",
        )?;
        check(
            num(p, "pull_length", 128.0)? > 0.0,
            "pull post spacing must be positive",
        )?;
        let top = kit.parameter(
            "hardware_top_offset",
            num(p, "hardware_top_offset", 88.9)?,
            "distance down from front top to hardware center when not centered",
        );
        let pull = kit.parameter(
            "pull_length",
            num(p, "pull_length", 128.0)?,
            "distance between pull post centers",
        );
        let projection = kit.parameter(
            "hardware_projection",
            num(p, "hardware_projection", 25.4)?,
            "hardware projection from front face",
        );
        Ok(Self {
            switches: Vec::new(),
            parameters,
            show,
            hardware,
            knobs,
            center,
            x,
            top,
            pull,
            projection,
        })
    }

    pub fn gate(&mut self, name: &str, condition: Value) {
        self.switches
            .push(json!({"feature":name,"condition":condition}));
    }

    pub fn fronts(&mut self, commands: &Commands) {
        for (command, args) in commands {
            let Some(name) = args["name"].as_str() else {
                continue;
            };
            if command.starts_with("solid.")
                && (name.contains(" Drawer Front")
                    || name.contains(" Door L")
                    || name.contains(" Door R")
                    || name.ends_with(" Front array"))
            {
                self.gate(name, is(&self.show, "eq", "1"));
            }
        }
    }

    pub fn shelves(&mut self, kit: &Kit, p: &Value) -> Result<(), String> {
        let count = num(p, "shelf_count", 1.0)?;
        check(
            (0.0..=6.0).contains(&count) && count.fract() == 0.0,
            "shelf count must be 0 to 6",
        )?;
        let n = number(
            kit,
            "shelf_count",
            count,
            "Number of evenly spaced shelves; tested 0 through 6",
        );
        self.parameters.push(n.clone());
        let id = |s| format!("{}_{s}", kit.prefix);
        let (w, h, d, t, pl, back) = (
            id("width"),
            id("height"),
            id("depth"),
            id("thickness"),
            id("plinth"),
            id("back_thickness"),
        );
        let pitch = format!("({h}-{pl}-{t})/({n}+1)");
        kit.panel(
            "Adjustable shelf",
            [&t, "0 mm", &format!("{pl}+{pitch}")],
            [&format!("{w}-2*{t}"), &format!("{d}-12 mm-{back}"), &t],
            None,
        );
        kit.push(("solid.pattern.rectangular",json!({"name":format!("{} Shelf array",kit.name),"bodies":[format!("{} Adjustable shelf",kit.name)],"dir1":[0,0,1],"count1":format!("max({n},1)"),"spacing1":pitch})));
        for part in ["Adjustable shelf", "Shelf array"] {
            self.gate(&format!("{} {part}", kit.name), is(&n, "ne", "0"));
        }
        Ok(())
    }

    pub fn handles(
        &mut self,
        kit: &Kit,
        name: &str,
        corner: [&str; 3],
        size: [&str; 2],
        right: Option<bool>,
        pattern: Option<(&str, &str)>,
    ) {
        let [x, y, z] = corner;
        let [w, h] = size;
        let cx = match right {
            None => format!("({x})+({w})/2+{}", self.x),
            Some(false) => format!("({x})+({w})-38.1 mm-{}", self.x),
            Some(true) => format!("({x})+38.1 mm+{}", self.x),
        };
        let cz = format!(
            "({z})+{}*({h})/2+(1-{})*(({h})-{})",
            self.center, self.center, self.top
        );
        for knob in [false, true] {
            let label = if knob { "Knob" } else { "Pull" };
            let mut bodies = Vec::new();
            let mut cylinder = |part: &str,
                                base: [String; 3],
                                axis: [i32; 3],
                                radius: &str,
                                height: &str| {
                let body = format!("{} {name} {label} {part}", kit.name);
                kit.push(("solid.cylinder",json!({"name":body,"body_name":body,"base":kit.at(&base[0],&base[1],&base[2]),"axis":axis,"radius":radius,"height":height})));
                bodies.push(body);
            };
            if knob {
                cylinder(
                    "stem",
                    [
                        cx.clone(),
                        format!("({y})-{}+6 mm", self.projection),
                        cz.clone(),
                    ],
                    [0, 1, 0],
                    "4 mm",
                    &format!("{}-6 mm", self.projection),
                );
                cylinder(
                    "head",
                    [cx.clone(), format!("({y})-{}", self.projection), cz.clone()],
                    [0, 1, 0],
                    "12.7 mm",
                    "6 mm",
                );
            } else {
                let horizontal = right.is_none();
                for (part, sign) in [("post A", "-"), ("post B", "+")] {
                    let bx = if horizontal {
                        format!("{cx}{sign}{}/2", self.pull)
                    } else {
                        cx.clone()
                    };
                    let bz = if horizontal {
                        cz.clone()
                    } else {
                        format!("{cz}{sign}{}/2", self.pull)
                    };
                    cylinder(
                        part,
                        [bx, format!("({y})-{}+5 mm", self.projection), bz],
                        [0, 1, 0],
                        "4 mm",
                        &format!("{}-5 mm", self.projection),
                    );
                }
                let bx = if horizontal {
                    format!("{cx}-{} /2-5 mm", self.pull)
                } else {
                    cx.clone()
                };
                let bz = if horizontal {
                    cz.clone()
                } else {
                    format!("{cz}-{} /2-5 mm", self.pull)
                };
                cylinder(
                    "bar",
                    [bx, format!("({y})-{}+5 mm", self.projection), bz],
                    if horizontal { [1, 0, 0] } else { [0, 0, 1] },
                    "5 mm",
                    &format!("{}+10 mm", self.pull),
                );
            }
            let condition = json!({"type":"all","conditions":[is(&self.show,"eq","1"),is(&self.hardware,"eq","1"),is(&self.knobs,"eq",if knob {"1"} else {"0"})]});
            for body in &bodies {
                self.gate(body, condition.clone());
            }
            if let Some((count, pitch)) = pattern {
                let feature = format!("{} {name} {label} array", kit.name);
                kit.push(("solid.pattern.rectangular",json!({"name":feature,"bodies":bodies,"dir1":[0,0,1],"count1":count,"spacing1":pitch})));
                self.gate(&feature, condition);
            }
        }
    }
}

pub fn install(run: Run, summary: &mut Value, ids: &BTreeMap<String, u64>) -> Result<(), String> {
    let Some(control) = summary
        .as_object_mut()
        .and_then(|s| s.remove("source_controls"))
    else {
        return Ok(());
    };
    let switches = control["switches"]
        .as_array()
        .ok_or("missing source switches")?;
    let names: Vec<_> = switches.iter().map(|s| s["feature"].clone()).collect();
    run(
        "config.configure",
        json!({"params":control["parameters"],"features":names}),
    )?;
    let mut rules = Vec::new();
    for (i, s) in switches.iter().enumerate() {
        let name = s["feature"].as_str().ok_or("missing feature name")?;
        let id = ids.get(name).ok_or(format!("no feature id for {name}"))?;
        for suppressed in [false, true] {
            rules.push(json!({"id":format!("sdh-{id}-{i}-{suppressed}"),"name":format!("{name}: {}",if suppressed {"hide"} else {"show"}),
                "condition":if suppressed {json!({"type":"not","condition":s["condition"]})} else {s["condition"].clone()},
                "effect":{"type":"drive_suppression","feature":id,"value":suppressed}}));
        }
    }
    run("config.rule", json!({"rules":rules}))?;
    // Snapshot the current design so adding a cabinet never resets earlier edits.
    let row = format!("{} controls", summary["name"].as_str().unwrap());
    run("config.row", json!({"name":row}))?;
    run("config.activate", json!({"row":row}))?;
    Ok(())
}
