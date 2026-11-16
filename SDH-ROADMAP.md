# SDH roadmap and upstream overlap

Owner: SDH family bridge work. Keep one active implementation slice and check upstream before starting it. This fork's tested baseline is **Woodwork v0.1.6**, not whichever revision main currently points to. Accept upgrades in an isolated test run before adopting them.

## First slice

- [x] Independent drawer / two-door layout and overlay / inset / slab / shaker / raised-panel selections.
- [x] Shared face-frame and front construction logic, ordinary vanilla SolidCraft features.
- [x] Actual editable drawer repetition, 1–4; inset rails follow.
- [x] Separate rails, stiles and recessed/raised panels for coordination.
- [x] Generic downstream parameter publication, source-driven resize and compatible reload.
- [ ] Cabinet-maker construction review: joints, rail/stile profile, panel retention and movement, materials, hardware and exposed-end standards.
- [ ] General parameter bounds/interference checks after arbitrary user edits.
- [ ] Live construction/style selectors only if they can preserve source dependencies and clearly expose supported choices.
- [ ] Expand wall/tall library variants after reviewing these base templates. Existing wall/tall drawing samples are distinct earlier fixtures.

## Expanded residential library target — November 16, 2026

The user's Revit reference library establishes the target: consistent flexibility across base, wall, tall and floating cabinets; single/double doors, drawers, mixed/open compartments, sink/appliance/corner layouts and accessories. Share authored compartment/front/construction rules across applicable styles. The first twelve base presets are a foundation, not equivalent coverage.

- [x] Adjustable shelves, toe-kick height/setback and source-driven visibility on the existing two base layouts.
- [ ] Compartment divisions and top-drawer independent heights.
- [x] Shared knob/pull selection, placement, centering and visibility on the existing two base layouts.
- [ ] Material/finish and inlay controls.
- [ ] Routine front-style changes, including independent upper/lower choices where applicable, preserving source dependencies and downstream overrides. Current styles remain generation-time presets.
- [ ] Crown front/left/right controls, crown height and wall-unit bottom trim.
- [ ] Common single-door, mixed door/drawer and open bases; wall and divided tall variants using the same style rules.
- [ ] Corner, sink/appliance, floating and supporting accessory families after upstream audits.
- [ ] Imperial/metric authoring and consistent dimensional meanings across templates.

Consumer-side drawing requirements include plan/elevation swing symbols and under-counter outlines, distinct from physical door angle and source geometry visibility. These require generic BIM integration rather than cabinet-specific engine rules. The generic BIMCraft Rust elevation edge defect remains the drawing-release prerequisite. No manufacturing/compliance acceptance follows from catalog coverage.

Private user references and the detailed coverage/acceptance matrix are retained in the parent workspace's `roadmap/rust/FAMILY-PUBLISHING.md`, section “Revit cabinet flexibility target — November 16, 2026”. Do not copy reference images into this public fork. This update defines direction only; retain one bounded implementation slice at a time.

## Upstream watch

Review the original [PLAN.md](https://github.com/TomasDJo/solidcraft-woodwork/blob/main/PLAN.md), releases and relevant issues/PRs. Existing SDH daily upstream watch includes this repository; it reports material changes and never automatically merges or upgrades.

| Pending upstream area | Relevance / SDH boundary |
| --- | --- |
| Inset doors | High overlap. Compare upstream clearance/hinges to our framed inset study; do not equate the two implementations automatically. |
| Corner, divider and shelf joinery; thin backs | Needed before fabrication-ready assemblies. Avoid duplicating it for the first base study. |
| Joint axes following resize | Needed before reliable door/drawer motion after dimensions change. New SDH base templates intentionally have no motion joints. |
| Continuous worktops and plinths | Project runs and adjacency; outside standalone cabinet templates. |
| Edge banding, grain, nesting/kerf | Manufacturing preparation; not implied by a plausible 3D model or bounding-box cut list. |
| Per-part DXF and labels | Cabinet-maker outputs after parts and machining definitions are approved. |
| SolidCraft preview, remembered dialogs and dependent fields | Improves generator usability; do not patch vanilla SolidCraft to fake these capabilities. |

Already released by the selected baseline: corner bi-fold and diagonal front work, handle position choices and SolidCraft 0.43 dropdown titles. Reuse upstream rather than putting those on the local backlog again. A roadmap checkbox is not runtime acceptance.

Latest upstream audit: Woodwork 0.1.7-rc.1, `42d08eed94ea6a576f1270b59425ffa93c085cae`, observed November 16, 2026. Edit cabinet, labels, auto corner, resize hinge fixes and CSV remain relevant. Not adopted; tested baseline stays 0.1.6.
