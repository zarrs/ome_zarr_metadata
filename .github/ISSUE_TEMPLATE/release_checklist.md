---
name: 🚀 Release Checklist
about: Track the steps for a new version release
title: "Release: vX.Y.Z"
labels: ["release"]
assignees: []
---

## Release Info
- **Version:** vX.Y.Z

## 1. Pre-Release Checks
- [ ] All intended feature PRs are merged into `main`
- [ ] CI/CD pipelines are passing for the latest commit on `main`
- [ ] Dependencies updated (where reasonable)
  - `cargo update --verbose` reveals out-of-date dependencies
- [ ] `Unreleased` section of `CHANGELOG.md` is up-to-date with unreleased changes
- [ ] `Cargo.toml` should already have the intended release version (should have a `-dev` suffix)

## 2. Release PR
- [ ] Create a branch `release/<version>`, e.g. `release/0.3.0`
- [ ] Update `version` in `Cargo.toml` (if required), e.g.
    ```diff
    - version = "0.3.0-dev"
    + version = "0.3.0"
    ```
- [ ] Update `CHANGELOG.md`
    - [ ] Add a new section under `## [Unreleased]` with the intended version / release date
        ```diff
        - ## [Unreleased]
        + ## [Unreleased]

        + ## [0.3.0] - 2025-11-26
        ```
    - [ ] Add a new link to the intended tag near the bottom of CHANGELOG.md. For example:
        ```diff
        +[0.3.0]: https://github.com/zarrs/ome_zarr_metadata/releases/tag/v0.3.0
        ```
    - [ ] Update the `[unreleased]` link near the bottom of CHANGELOG.md. For example:
        ```diff
        -[unreleased]: https://github.com/zarrs/ome_zarr_metadata/compare/v0.2.5...HEAD
        +[unreleased]: https://github.com/zarrs/ome_zarr_metadata/compare/v0.3.0...HEAD
        ```
- [ ] Commit changes and create a release PR with name: `chore: release <version>`
- [ ] Get an approval from another maintainer
- [ ] Squash merge release PR

## 3. Post-Release Verification
- [ ] Verify `publish` action completed succesfully
- [ ] Verify that `docs.rs` build completed succesfully

## 4. Announce
- [ ] Announce the GitHub release (TODO: automate?)

## 5. Version Increment
- [ ] Increment the minor version on `main` and add a `-dev` suffix, e.g. `0.3.1-dev`

## 6. Done!
- [ ] Close this issue