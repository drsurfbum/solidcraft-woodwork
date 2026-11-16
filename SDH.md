# Base cabinet contract — first SDH slice

Baseline: upstream v0.1.6, `b905965224a4378e2f063faf92278b0fbd9bbfce`. Runtime reference: official vanilla SolidCraft 0.43.0. One source design per cabinet/family. Layout rules live in this plugin, normal SolidCraft parameters/features own the resulting model, and consuming BIM software evaluates the saved source generically.

## What the generator produces

| Layout | Overlay | Inset |
| --- | --- | --- |
| Drawer base, editable count 1–4 | Slab, shaker, raised panel | Slab, shaker, raised panel |
| Two-door base, fixed pair and one shelf | Slab, shaker, raised panel | Slab, shaker, raised panel |

Both construction choices use a face-frame carcass. Overlay fronts cover the face frame; inset fronts fit within its opening, flush with the frame face and separated by the authored reveal. Inset drawers include divider rails that follow the count. This is a defined first construction standard, not a claim to cover frameless overlay or beaded inset.

Slab is one front body. Shaker and raised-panel fronts have separate stile, rail and panel bodies. Shaker panels are recessed; raised panels have an authored lofted field. Rails/stiles meet with simple butt interfaces and the panel has no fabrication tongue/groove or movement allowance. Do not treat this as a floating-panel manufacturing specification.

Defaults are explicit study inputs, not a cabinet maker's approved standards: 609.6 W × 876.3 H × 609.6 mm carcass depth; 101.6 mm toe kick; 38.1 mm face frame; 50.8 mm front frame; 2 mm reveal. Default frame/front depth is 19.05 mm. Overall depth includes the frame and, for overlay, its gap and front thickness. Report carcass and overall depth separately.

## Interaction and parameter contract

1. Choose **SDH Base Cabinet**, layout, construction and style; enter dimensions in millimetres and generate.
2. Edit the saved design's ordinary parameters. Width, height, depth, reveals and frame dimensions drive the model. Drawer count changes an ordinary rectangular pattern, including inset rails; no rerun of the plugin is needed for counts 1–4.
3. On generic family publication, the author chooses category, instance/type scopes, Properties groups and schedule aliases. Suggested demonstration mapping: Width and Drawer count = Instance; Height, Depth and relevant construction sizes = Type; Count under Graphics; lengths under Dimensions. Helpers that do not drive the selected template need not be published.
4. Construction/style are **generation-time choices**, not exposed text fields pretending to change topology. Generate a replacement source and republish to switch a placed family. Stable shared input names allow the consumer to preserve widths/counts and placement identities during compatible reload.
5. Save and reopen the source without the plugin: all editable model logic remains in vanilla SolidCraft features. The plugin is required to generate new templates, not to evaluate existing ones.

Generation rejects invalid layouts/styles, fractional/out-of-range initial counts and undersized fronts/panels. Later parameter edits use SolidCraft's feature validation; general maker-specific bounds and interference validation are future work. The tested resizing range is representative, not a guarantee for every combination of dimensions.

## Acceptance

`cargo test`: 23 unit tests. `cargo clippy --all-targets -- -D warnings` and rustfmt checks. Upstream end-to-end tests retained; additional `tests/test_base.py` exercises all 12 templates, 30 edited dimension/count states, reopen, positive solid volumes, drawer counts, frame reveals and panel depth. The built package is tested in vanilla SC43.

The consuming BIMCraft Rust working build was separately checked against independent SC43 evaluations: 12 fresh family publications, 30 independent instance changes (counts 1–4 for each drawer preset), shared type height/depth, schedule aliases, unchanged peers, stable IDs, Undo/Redo and reopen. A slab→raised-panel source reload preserves distinct widths/counts. Repository tests remain independent of BIMCraft and private source files.

Revit references: user-provided casework Properties/Edit Type examples; Autodesk **Revit 2025** [Create Family Parameters](https://help.autodesk.com/cloudhelp/2025/ENU/Revit-Customize/files/GUID-921F7A15-D191-4F75-8243-4989C482E253.htm) and [Family Types](https://help.autodesk.com/cloudhelp/2025/ENU/Revit-Customize/files/GUID-7C0C2F22-8E85-4576-BD84-D710AC6D4B6F.htm). These inform the receiving family workflow; this plugin does not implement Revit's family editor.

| Behavior | Current result |
| --- | --- |
| Instance size/count versus shared type dimensions | Supported through source-backed publication; scope chosen by author |
| Independent layout, construction and style choices | Supported at generation; two layouts × six combinations |
| Count changes rebuild actual parts | Supported and tested 1–4 drawers, including inset rails |
| Editable enum changing style inside BIM Properties | Not implemented; explicit source preset and republish |
| Save, reopen and compatible source reload | Tested on source and receiving projects |
| Hardware, opening motion and fabrication joinery | Not part of the new base templates; existing upstream cabinet entry remains available |
| Native associative drawings and production cut lists | Not accepted by this slice; coordination sheets are a separate workflow |

Known downstream view limitation: BIMCraft Rust 0.12 working-build 2D elevations can omit edges on the new fronts made from separate parts. Source and 3D geometry tests pass. Comparison previews use front-facing 3D renders; do not substitute those for accepted native technical elevations.

No arbitrary-family or full Revit compatibility claim follows from these cases. Source models and native screenshots used for the SDH work are synthetic.
