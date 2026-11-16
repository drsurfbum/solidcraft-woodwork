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
