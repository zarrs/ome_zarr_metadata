# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3] - 2024-12-10

### Added
- Implement `Default` for `v0_4::OmeNgffGroupAttributes` and `v0_5::OmeFields`

### Fixed
- Fix incorrect labels field in `v0_4::OmeNgffGroupAttributes` and `v0_5::OmeFields`

## [0.2.2] - 2024-12-10

*This release was yanked.*

### Added
- Add top-level OME-NGFF/OME-Zarr structs
  - Add `v0_4::OmeNgffGroupAttributes`
  - Add `v0_5::{OmeFields,OmeZarrGroupAttributes,OmeZarrGroupMetadata}`

## [0.2.1] - 2024-12-07

### Added
- Implement `Clone` for all structs

## [0.2.0] - 2024-11-23

### Added
- Add OME-Zarr version `0.5` (22 November 2024)

### Removed
- Remove the OME-Zarr `0.5-dev1` interim version https://github.com/ome/ngff/pull/249
- Remove the OME-Zarr `0.5-dev` interim version
- Remove the OME-Zarr `0.5+RFC-2` interim version https://github.com/ome/ngff/pull/242

## [0.1.0] - 2024-07-24

### Added
- Initial public release

[unreleased]: https://github.com/LDeakin/rust_ome_zarr_metadata/compare/v0.2.3...HEAD
[0.2.3]: https://github.com/LDeakin/rust_ome_zarr_metadata/releases/tag/v0.2.3
[0.2.2]: https://github.com/LDeakin/rust_ome_zarr_metadata/releases/tag/v0.2.2
[0.2.1]: https://github.com/LDeakin/rust_ome_zarr_metadata/releases/tag/v0.2.1
[0.2.0]: https://github.com/LDeakin/rust_ome_zarr_metadata/releases/tag/v0.2.0
[0.1.0]: https://github.com/LDeakin/rust_ome_zarr_metadata/releases/tag/v0.1.0
