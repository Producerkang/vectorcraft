# Exam Notched arrowhead (personal fork)

This fork adds the **Exam Notched** stroke arrowhead (a filled six-vertex, V-notched exam-style head) without replacing any of the 40 original arrowheads.

- The new variant is appended as `Arrowhead::ExamNotched` to preserve older saved variant names.
- Geometry is generated with the existing arrowhead pipeline, so both line endpoints, scaling, PDF/SVG export, selection and rendering use shared infrastructure.
- The six vertices approximate the Illustrator screenshot reference; they are not the original vector geometry.
- Only this fork understands the `ExamNotched` variant in native `.vectorcraft` documents. For exchanging finalized artwork with official builds, export SVG or PDF.
- Official in-app self-update checks are disabled to prevent replacing this custom binary. To update, merge upstream changes into this fork, re-run tests, and rebuild. Do not install official binaries over the custom executable.

## Tests

```sh
cargo test -p vectorcraft-doc
cargo test -p vectorcraft-effects
cargo test -p vectorcraft-ui-egui
cargo test -p vectorcraft-pdf
cargo test -p vectorcraft-svg
cargo build --release -p vectorcraft
```

Original copyright, license and NOTICE files remain unchanged. The custom variant appears in the Stroke panel's arrowhead menu.

## Windows portable build from GitHub Actions

On this fork open **Actions** and enable workflows if GitHub asks. Run **Exam Notched Windows Build**, or view the run created by a push/PR. After a successful run, download the `VectorCraft-Exam-Notched-Windows-x64` artifact and extract the whole directory outside the official installation folder. Launch `vectorcraft.exe` from that directory. The `portable.txt` marker keeps local settings in a `VectorCraftData` sibling directory. This custom executable is unsigned; Windows may display a publisher warning. Only run binaries you built in your own repository.

The Windows build workflow is intended for x64 PCs; it does not automatically install or replace the official VectorCraft program.
