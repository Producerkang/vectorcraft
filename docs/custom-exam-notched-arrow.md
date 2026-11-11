# Exam Notched arrowhead (personal fork)

This fork adds the **Exam Notched** stroke arrowhead (a filled six-vertex, V-notched exam-style head) without replacing any of the 40 original arrowheads.

- The new variant is appended as `Arrowhead::ExamNotched` to preserve older saved variant names.
- Geometry is generated with the existing arrowhead pipeline, so both line endpoints, scaling, PDF/SVG export, selection and rendering use shared infrastructure.
- The six vertices approximate the Illustrator screenshot reference; they are not the original vector geometry.
- Only this fork understands the `ExamNotched` variant in native `.vectorcraft` documents. For exchanging finalized artwork with official builds, export SVG or PDF.
- Official in-app releases are not used. The custom build checks only **Producerkang/vectorcraft** Releases, but unsigned Windows executables intentionally offer **DOWNLOAD**, not unverified self-installation. Never install an official binary over the personal executable.

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

Until the first personal Release is published, use **Actions → Exam Notched Windows Build** to download the artifact. After the custom update channel publishes a Release, use **Releases** for the current tested version. Extract the whole portable directory outside the official installation folder. Launch `vectorcraft.exe` from that directory. The `portable.txt` marker keeps local settings in a `VectorCraftData` sibling directory. This custom executable is unsigned; Windows may display a publisher warning. Only run binaries you built in your own repository.

The Windows build workflow is intended for x64 PCs; it does not automatically install or replace the official VectorCraft program.

## Automatic personal release channel (unsigned, download-only)

The separate `.github/workflows/custom-update-channel.yml` workflow runs when personal `main` changes and daily. It checks GitHub's latest **published stable official release**, merges that tag into an isolated candidate, and stops on source conflicts. Only if the custom arrowhead and fork update source survive and Windows formatting, selected tests, and build pass does it advance personal `main` and publish a personal Release. Failed runs leave the last published personal Release intact.

Personal Release versions use `100.0.<run-number>` to maintain a separate monotonically increasing channel; they are **not** claims about official VectorCraft versions. Each Release identifies the official source tag it includes and publishes `vectorcraft-<version>-windows-x64-portable.zip` plus `SHA256SUMS.txt`. Temporary Actions artifacts expire after seven days; public Releases are long-lived.

The app's **Help → Check for Updates** and startup update check use only personal Releases. A custom Release appears with **DOWNLOAD** and opens its GitHub page. The original craft-update Windows updater requires trusted Authenticode signatures for both existing and new executables. We do **not** bypass that security requirement: automatic in-place installation will need a separate signing and verification design.

For the first transition from the old unsigned custom app (which has update checks disabled), manually download the newest personal Release once. Preserve `portable.txt` next to `vectorcraft.exe` and keep the customized copy in its own writable folder. Do not overwrite the official installation. Settings stay in the sibling `VectorCraftData` directory.

**Attribution and distribution:** The source is MIT OR Apache-2.0 and its other notices are retained, but official ArtCraft branding and logo assets are not part of the source license. Publishing a portable build on a public GitHub repo is redistribution; review branding and third-party notice requirements before distributing it beyond personal use.
