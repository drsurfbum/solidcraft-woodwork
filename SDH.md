# Base cabinet contract — first SDH slice

Baseline: upstream v0.1.6, `b905965224a4378e2f063faf92278b0fbd9bbfce`. Runtime reference: official vanilla SolidCraft 0.43.0. One source design per cabinet/family. Layout rules live in this plugin, normal SolidCraft parameters/features own the resulting model, and consuming BIM software evaluates the saved source generically.

## What the generator produces

| Layout | Overlay | Inset |
| --- | --- | --- |
| Drawer base, editable count 1–4 | Slab, shaker, raised panel | Slab, shaker, raised panel |
| Two-door base, fixed pair and adjustable shelves | Slab, shaker, raised panel | Slab, shaker, raised panel |

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

## Shared controls — SDH 0.1.6-sdh.2

The same twelve presets now include:

- `show_fronts`, `show_hardware`, `use_knobs`, `center_hardware`: ordinary dimensionless 0/1 inputs, with source-authored configuration suppression. Hiding fronts retains drawers/interiors and hides attached hardware. Inactive hardware variants are suppressed solids, not invisible exported meshes.
- `shelf_count`: two-door interior, tested 0–6 evenly spaced shelves. Drawer count remains 1–4. No generator rerun is needed.
- `plinth` height and `toe_kick_setback`: genuine recessed toe kick and notched side panels. Setback is measured from the carcass front, excluding frame/front projection.
- `hardware_x_offset`: drawer shift from center; door inward shift from a 38.1 mm meeting-edge baseline. `hardware_top_offset`: down from front top to hardware center when vertical centering is off. `pull_length`: post spacing; `hardware_projection`: projection from the front.

Default hardware is a 128 mm bar pull, 25.4 mm projection, vertically centered. Turning centering off uses 88.9 mm from the top. Knobs are a simple 25.4 mm diameter coordination shape. Source material assignment retains the plugin's existing board material behavior; these are not manufacturer-specific fittings or machining definitions.

Source configuration rows/rules are saved in vanilla SolidCraft 0.43. Adding another cabinet snapshots the current design rather than restoring earlier Default-row values. This prevents adding a new cabinet from resetting prior visibility edits. Publication still requires one standalone cabinet per source design.

The generic receiving importer explicitly maps source dimensionless inputs to Number, Whole number or Yes/No; it does not infer control kinds from names. Authors retain scope/group choices. Demonstration counts/visibility/offsets/toe kick/width are Instance, other construction and hardware sizes Type. Source defaults/descriptions remain visible during review.

Only generator-time ranges and native feature validity are currently enforced. Arbitrary subsequent edits need further bounds/interference work: negative hardware projection is geometrically evaluable but physically inappropriate. Do not interpret successful evaluation as fabrication validation.

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
| Hardware / opening motion / fabrication joinery | Coordination knob/pull geometry and source-driven placement/visibility supported; motion and fabrication joinery not accepted |
| Native associative drawings and production cut lists | Not accepted by this slice; coordination sheets are a separate workflow |

The receiving BIMCraft Rust elevation fix tracks disconnected coplanar mesh patches separately, restoring subpixel gaps on multi-part fronts. It is a generic receiving-app change and does not change source geometry. See the parent workspace cabinet-controls-2026-11-16 evidence for its regression and exact build.

No arbitrary-family or full Revit compatibility claim follows from these cases. Source models and native screenshots used for the SDH work are synthetic.
