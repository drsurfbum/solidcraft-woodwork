//! SDH base-cabinet studies. All geometry and repetition live in ordinary SolidCraft
//! features. Construction/style select a source template; dimensions and drawer count
//! remain authoring parameters. No BIMCraft-specific geometry or parameter lookup.
pub(crate) mod controls;
use crate::{
    cabinet,
    kit::{check, mm, num, Kit},
    Commands,
};
use serde_json::{json, Value};

pub fn build(p: &Value) -> Result<(Commands, Value), String> {
    check(
        p["units"].as_str().unwrap_or("mm") == "mm",
        "SDH base sizes are in millimetres",
    )?;
    let kit = Kit::new(p)?;
    let layout = p["layout"].as_str().unwrap_or("drawers");
    let construction = p["construction"].as_str().unwrap_or("overlay");
    let style = p["style"].as_str().unwrap_or("slab");
    check(
        matches!(layout, "drawers" | "two-door"),
        "layout must be drawers or two-door",
    )?;
    check(
        matches!(construction, "overlay" | "inset"),
        "construction must be overlay or inset",
    )?;
    check(
        matches!(style, "slab" | "shaker" | "raised-panel"),
        "style must be slab, shaker or raised-panel",
    )?;
    let count = num(p, "drawer_count", 3.0)?;
    check(
        (1.0..=4.0).contains(&count) && count.fract() == 0.0,
        "drawer count must be 1 to 4",
    )?;
    let size = |key, default| kit.size(p, key, default);
    let (w, h, d) = (
        size("width", 609.6)?,
        size("height", 876.3)?,
        size("depth", 609.6)?,
    );
    let (t, pl) = (size("thickness", 18.0)?, size("plinth", 101.6)?);
    check(pl > 0.0, "SDH bases require a positive toe-kick height")?;
    let f = size("face_frame_width", 38.1)?;
    let fd = size("face_frame_depth", 19.05)?;
    let ft = size("front_thickness", 19.05)?;
    let r = size("reveal", 2.0)?;
    let ff = size("front_frame_width", 50.8)?;
    let pt = size("panel_thickness", 6.35)?;
    let recess = size("panel_recess", 6.35)?;
    let bevel = size("panel_bevel", 12.7)?;
    check(
        f >= t && f <= 100.0 && fd >= ft && (12.0..=40.0).contains(&ft),
        "face frame must cover carcass; frame depth >= front thickness (12–40 mm)",
    )?;
    check(
        r > 0.0 && r <= 10.0 && ff >= 20.0,
        "reveal must be >0 to 10 mm and front frame at least 20 mm",
    )?;
    check(
        pt >= 3.0 && recess > 0.0 && pt + recess <= ft && bevel > 0.0,
        "panel thickness plus recess must fit the front; bevel must be positive",
    )?;
    let inset = construction == "inset";
    let n = if layout == "drawers" { count } else { 1.0 };
    let fw = if inset {
        w - 2.0 * f - 2.0 * r
    } else {
        w - 2.0 * r
    };
    let fh = if inset {
        (h - pl - f) / n - f - 2.0 * r
    } else {
        (h - pl - (n + 1.0) * r) / n
    };
    let piece_w = if layout == "two-door" {
        (fw - r) / 2.0
    } else {
        fw
    };
    check(fw > 100.0 && fh > 90.0, "cabinet opening is too small")?;
    check(
        style == "slab" || (piece_w > 2.0 * (ff + bevel) + 10.0 && fh > 2.0 * (ff + bevel) + 10.0),
        "front is too small for its frame and panel bevel",
    )?;
    // Reuse the upstream carcass/back/shelf builder without upstream front hardware or
    // motion joints. Inset shelves start behind the face frame, which lies in front of Y=0.
    let mut carcass = p.clone();
    for (key, value) in [
        ("width", w),
        ("height", h),
        ("depth", d),
        ("thickness", t),
        ("plinth", pl),
    ] {
        carcass[key] = json!(value);
    }
    carcass["units"] = json!("mm");
    carcass["drawers"] = json!(0);
    carcass["doors"] = json!(0);
    carcass["dividers"] = json!(0);
    carcass["shelves"] = json!(0);
    carcass["back"] = json!("groove");
    carcass["shelf_pins"] = json!(false);
    carcass["handles"] = json!("none");
    let (mut commands, mut summary) = cabinet::build(&carcass)?;
    let setback = num(p, "toe_kick_setback", 76.2)?;
    check(
        setback > 0.0 && setback < d - 50.0,
        "toe-kick setback must leave at least 50 mm of cabinet depth",
    )?;
    let es = kit.parameter(
        "toe_kick_setback",
        setback,
        "toe-kick recess measured back from the carcass front, excluding fronts",
    );
    for (cmd, args) in &mut commands {
        if *cmd == "solid.box" && args["name"] == format!("{} Plinth", kit.name) {
            args["corner"][1] = json!(format!("{es}-{} /2", kit.prefix.clone() + "_depth"));
        }
    }
    let mut controls = controls::Controls::new(&kit, p, layout == "drawers")?;
    let id = |key: &str| format!("{}_{key}", kit.prefix);
    let (ew, eh, ed, epl) = (id("width"), id("height"), id("depth"), id("plinth"));
    kit.centre(&ew, &ed);
    let ef = kit.parameter("face_frame_width", f, "face-frame stile/rail width");
    let efd = kit.parameter(
        "face_frame_depth",
        fd,
        "face frame depth in front of carcass",
    );
    let eft = kit.parameter("front_thickness", ft, "door/drawer front thickness");
    let er = kit.parameter("reveal", r, "clear gap around fronts");
    let eff = kit.parameter("front_frame_width", ff, "door/drawer stile and rail width");
    let ept = kit.parameter("panel_thickness", pt, "recessed panel backing thickness");
    let erecess = kit.parameter("panel_recess", recess, "panel face recess from front face");
    let ebevel = kit.parameter("panel_bevel", bevel, "raised panel bevel run");
    let en = id("drawer_count");
    if layout == "drawers" {
        kit.push(("parameters.add",json!({"name":en,"expression":count.to_string(),"unit":"","comment":"Number of equal drawers; tested 1 through 4"})));
    }
    let zero = mm(0.0);
    let sides = [
        format!("{} Side L", kit.name),
        format!("{} Side R", kit.name),
    ];
    kit.panel(
        "Toe-kick notch",
        [&zero, &zero, &zero],
        [&ew, &es, &epl],
        Some(&sides),
    );
    if layout == "two-door" {
        controls.shelves(&kit, p)?;
    }
    let frame_y = format!("-{efd}");
    let inner_w = format!("{ew}-2*{ef}");
    kit.panel(
        "Face frame left",
        [&zero, &frame_y, &epl],
        [&ef, &efd, &format!("{eh}-{epl}")],
        None,
    );
    kit.panel(
        "Face frame right",
        [&format!("{ew}-{ef}"), &frame_y, &epl],
        [&ef, &efd, &format!("{eh}-{epl}")],
        None,
    );
    kit.panel(
        "Face frame bottom",
        [&ef, &frame_y, &epl],
        [&inner_w, &efd, &ef],
        None,
    );
    kit.panel(
        "Face frame top",
        [&ef, &frame_y, &format!("{eh}-{ef}")],
        [&inner_w, &efd, &ef],
        None,
    );
    let pitch = if inset {
        format!("({eh}-{epl}-{ef})/{en}")
    } else {
        format!("({eh}-{epl}-{er})/{en}")
    };
    if inset && layout == "drawers" {
        kit.push(("solid.pattern.rectangular",json!({"bodies":[format!("{} Face frame bottom",kit.name)],"dir1":[0,0,1],"count1":en,"spacing1":pitch,"name":format!("{} Face frame rails",kit.name)})));
    }
    let x = if inset {
        format!("{ef}+{er}")
    } else {
        er.clone()
    };
    let y = if inset {
        frame_y.clone()
    } else {
        format!("-{efd}-{er}-{eft}")
    };
    let z = if inset {
        format!("{epl}+{ef}+{er}")
    } else {
        format!("{epl}+{er}")
    };
    let width = if inset {
        format!("{ew}-2*{ef}-2*{er}")
    } else {
        format!("{ew}-2*{er}")
    };
    let height = if layout == "drawers" {
        if inset {
            format!("{pitch}-{ef}-2*{er}")
        } else {
            format!("{pitch}-{er}")
        }
    } else if inset {
        format!("{eh}-{epl}-2*{ef}-2*{er}")
    } else {
        format!("{eh}-{epl}-2*{er}")
    };
    let front = Front {
        kit: &kit,
        style,
        thickness: &eft,
        frame: &eff,
        panel: &ept,
        recess: &erecess,
        bevel: &ebevel,
    };
    if layout == "two-door" {
        let dw = format!("({width}-{er})/2");
        front.build("Door L", [&x, &y, &z], &dw, &height);
        let rx = format!("{x}+{dw}+{er}");
        front.build("Door R", [&rx, &y, &z], &dw, &height);
        controls.handles(
            &kit,
            "Hardware L",
            [&x, &y, &z],
            [&dw, &height],
            Some(false),
            None,
        );
        controls.handles(
            &kit,
            "Hardware R",
            [&rx, &y, &z],
            [&dw, &height],
            Some(true),
            None,
        );
    } else {
        let fronts = front.build("Drawer Front", [&x, &y, &z], &width, &height);
        kit.push(("solid.pattern.rectangular",json!({"bodies":fronts,"dir1":[0,0,1],"count1":en,"spacing1":pitch,"name":format!("{} Front array",kit.name)})));
        controls.handles(
            &kit,
            "Hardware",
            [&x, &y, &z],
            [&width, &height],
            None,
            Some((&en, &pitch)),
        );
        let mut repeated = Vec::new();
        let slide = kit.parameter("slide_clearance", 12.7, "drawer slide clearance each side");
        let box_t = kit.parameter(
            "drawer_box_thickness",
            12.7,
            "drawer box side/front/back thickness",
        );
        let rear = kit.parameter(
            "drawer_back_clearance",
            10.0,
            "drawer box clearance from back panel",
        );
        // Boxes clear the face-frame opening as well as carcass sides. Their bottom is
        // above the bottom rail and front; depth is measured from the front's back face.
        let bx = format!("{ef}+{slide}");
        let bw = format!("{ew}-2*({ef}+{slide})");
        let by = format!("{y}+{eft}");
        let bz = if inset {
            format!("{z}+15 mm")
        } else {
            format!("{z}+{ef}+15 mm")
        };
        let bh = format!("{height}-2*{ef}-30 mm");
        let back = id("back_thickness");
        let bd = format!("{ed}-12 mm-{back}-{rear}-({by})");
        for (part, corner, size) in [
            (
                "Left",
                [bx.clone(), by.clone(), bz.clone()],
                [box_t.clone(), bd.clone(), bh.clone()],
            ),
            (
                "Right",
                [format!("{bx}+{bw}-{box_t}"), by.clone(), bz.clone()],
                [box_t.clone(), bd.clone(), bh.clone()],
            ),
            (
                "Box front",
                [format!("{bx}+{box_t}"), by.clone(), bz.clone()],
                [format!("{bw}-2*{box_t}"), box_t.clone(), bh.clone()],
            ),
            (
                "Box back",
                [
                    format!("{bx}+{box_t}"),
                    format!("{by}+{bd}-{box_t}"),
                    bz.clone(),
                ],
                [format!("{bw}-2*{box_t}"), box_t.clone(), bh.clone()],
            ),
            (
                "Bottom",
                [format!("{bx}+{box_t}"), format!("{by}+{box_t}"), bz.clone()],
                [
                    format!("{bw}-2*{box_t}"),
                    format!("{bd}-2*{box_t}"),
                    ept.clone(),
                ],
            ),
        ] {
            let part = format!("Drawer {part}");
            kit.panel(
                &part,
                [&corner[0], &corner[1], &corner[2]],
                [&size[0], &size[1], &size[2]],
                None,
            );
            repeated.push(format!("{} {part}", kit.name));
        }
        kit.push(("solid.pattern.rectangular",json!({"bodies":repeated,"dir1":[0,0,1],"count1":en,"spacing1":pitch,"name":format!("{} Drawer array",kit.name)})));
    }
    let (extra, parts) = kit.finish(Vec::new(), [w, h, d]);
    commands.extend(extra);
    // User inputs must exist before any feature that references them.
    commands.sort_by_key(|c| c.0 != "parameters.add");
    controls.fronts(&commands);
    summary["source_controls"] =
        json!({"parameters":controls.parameters,"switches":controls.switches});
    commands.retain(|c| c.0 != "timeline.group");
    let features: Vec<Value> = commands
        .iter()
        .filter(|c| c.0.starts_with("solid.") || c.0 == "sketch.create")
        .filter_map(|c| c.1.get("name").cloned())
        .collect();
    commands.push((
        "timeline.group",
        json!({"name":kit.name,"features":features}),
    ));
    summary["bodies"]
        .as_array_mut()
        .unwrap()
        .extend(parts["bodies"].as_array().unwrap().iter().cloned());
    summary["construction"] = json!(construction);
    summary["style"] = json!(style);
    summary["layout"] = json!(layout);
    summary["scope"]=json!("Coordination study: fixed construction/style template; editable dimensions and drawer count; no motion/hardware/joinery approval");
    // Fail before mutating the document if a future extension exceeds the host budget.
    check(
        commands.len() < 230,
        "base cabinet exceeds the plugin command budget",
    )?;
    Ok((commands, summary))
}

struct Front<'a> {
    kit: &'a Kit,
    style: &'a str,
    thickness: &'a str,
    frame: &'a str,
    panel: &'a str,
    recess: &'a str,
    bevel: &'a str,
}
impl Front<'_> {
    fn build(&self, name: &str, [x, y, z]: [&str; 3], w: &str, h: &str) -> Vec<String> {
        let Self {
            kit,
            style,
            thickness: t,
            frame: f,
            panel: p,
            recess: r,
            bevel: b,
        } = self;
        if *style == "slab" {
            kit.panel(name, [x, y, z], [w, t, h], None);
            return vec![format!("{} {name}", kit.name)];
        }
        let mut bodies = Vec::new();
        for (part, corner, size) in [
            (
                "Stile L",
                [x.to_string(), y.to_string(), z.to_string()],
                [f.to_string(), t.to_string(), h.to_string()],
            ),
            (
                "Stile R",
                [format!("({x})+({w})-{f}"), y.into(), z.into()],
                [f.to_string(), t.to_string(), h.into()],
            ),
            (
                "Rail bottom",
                [format!("({x})+{f}"), y.into(), z.into()],
                [format!("({w})-2*{f}"), t.to_string(), f.to_string()],
            ),
            (
                "Rail top",
                [format!("({x})+{f}"), y.into(), format!("({z})+({h})-{f}")],
                [format!("({w})-2*{f}"), t.to_string(), f.to_string()],
            ),
        ] {
            let part = format!("{name} {part}");
            kit.panel(
                &part,
                [&corner[0], &corner[1], &corner[2]],
                [&size[0], &size[1], &size[2]],
                None,
            );
            bodies.push(format!("{} {part}", kit.name));
        }
        let panel = format!("{name} Panel");
        let body = format!("{} {panel}", kit.name);
        kit.panel(
            &panel,
            [
                &format!("({x})+{f}"),
                &format!("({y})+{r}"),
                &format!("({z})+{f}"),
            ],
            [&format!("({w})-2*{f}"), p, &format!("({h})-2*{f}")],
            None,
        );
        if *style == "raised-panel" {
            for (label, inset, depth) in [
                ("base", f.to_string(), format!("-({y})-{r}-0.1 mm")),
                ("face", format!("{f}+{b}"), format!("-({y})")),
            ] {
                kit.push(("sketch.create",json!({"name":format!("{body} {label}"),"plane":{"origin":[0,0,0],"x_dir":[1,0,0],"y_dir":[0,0,1]},"offset":format!("({depth}) + {}/2",kit.prefix.clone()+"_depth")})));
                let shifted = kit.at(
                    &format!("({x})+({inset})"),
                    "0 mm",
                    &format!("({z})+({inset})"),
                );
                let other = kit.at(
                    &format!("({x})+({w})-({inset})"),
                    "0 mm",
                    &format!("({z})+({h})-({inset})"),
                );
                kit.push((
                    "sketch.rectangle.two_point",
                    json!({"p0":[shifted[0],shifted[2]],"p1":[other[0],other[2]]}),
                ));
                kit.push(("sketch.finish", json!({})));
            }
            kit.push(("solid.loft",json!({"name":format!("{body} raised field"),"sections":[{"sketch":format!("{body} base")},{"sketch":format!("{body} face")}],"operation":"join","targets":[body]})));
        }
        bodies.push(body);
        bodies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn presets_and_budget() {
        for layout in ["drawers", "two-door"] {
            for construction in ["overlay", "inset"] {
                for style in ["slab", "shaker", "raised-panel"] {
                    let (cmd, s) =
                        build(&json!({"layout":layout,"construction":construction,"style":style}))
                            .unwrap();
                    assert!(cmd.len() < 230);
                    assert_eq!(s["construction"], construction);
                    assert_eq!(s["style"], style);
                }
            }
        }
    }
    #[test]
    fn invalid_inputs() {
        for p in [
            json!({"drawer_count":0}),
            json!({"drawer_count":1.5}),
            json!({"construction":"guess"}),
            json!({"style":"unknown"}),
            json!({"panel_thickness":19,"panel_recess":6}),
            json!({"width":100}),
            json!({"layout":"drawers","drawer_count":4,"height":400}),
        ] {
            assert!(build(&p).is_err(), "{p}");
        }
    }
}
