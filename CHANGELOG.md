# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Validation and conformance tests ([#12] by [@clbarnes])
  - N.B. a number of tests are excluded as they do not match the specification text, see <https://github.com/ome/ngff/issues/325>
- Implement omero metadata (by [@clbarnes])
  - Add `Omero` type
  - Add `omero` field to `OmeFields` and `OmeNgffGroupAttributes`
- Add `Result` and `Error` types (by [@clbarnes])
- Add `MaybeNDim` and `NDim` traits for types with dimensionality (by [@clbarnes])
- Implement `From<AxisUnitSpace>` and `From<AxisUnitTime>` for `AxisUnit` (by [@clbarnes])

### Changed

- **Breaking**: Rename `Bioformats2rawLayout` to `Bioformats2Raw` (by [@clbarnes])
  - **Breaking**: Rename `OmeNgffGroupAttributes::bioformats2raw_layout` to `bioformats2raw`
  - **Breaking**: Rename `OmeFields::bioformats2raw_layout` to `bioformats2raw`
- **Breaking**: Make the `colors` field of `ImageLabel` an `Option` to match the specification (by [@clbarnes])
- **Breaking**: Rename `PlateAcquisition` fields to snake case (by [@clbarnes])
- **Breaking**: Increase MSRV to 1.82 (by [@clbarnes])

### Fixed

- Deserialisation of custom axis types in `AxisType` (by [@clbarnes])
- Permit unknown fields in `v0_5::MultiscaleImage` (by [@clbarnes])
  - See https://github.com/ome/ngff/issues/209

[#12]: https://github.com/zarrs/ome_zarr_metadata/pull/12

## [0.2.5] - 2025-08-29

### Added

- Implement conversion of v0.4->v0.5 ([#8] by [@clbarnes])

### Fixed

- Publicise some formerly-private fields in `ImageLabelColor`, `ImageLabelProperties`, and `ImageLabelSource` ([#9] by [@clbarnes])

[#8]: https://github.com/zarrs/ome_zarr_metadata/pull/8
[#9]: https://github.com/zarrs/ome_zarr_metadata/pull/9

## [0.2.4] - 2025-08-01

### Added

- Add trusted publishing

### Fixed

- Fix improper serialisation of `rowIndex` and `columnIndex` in `PlateWell` metadata
- Fix improper serialisation of `label-value` in `ImageLabel` metadata

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

[unreleased]: https://github.com/zarrs/ome_zarr_metadata/compare/v0.2.5...HEAD
[0.2.5]: https://github.com/zarrs/ome_zarr_metadata/releases/tag/v0.2.5
[0.2.4]: https://github.com/zarrs/ome_zarr_metadata/releases/tag/v0.2.4
[0.2.3]: https://github.com/zarrs/ome_zarr_metadata/releases/tag/v0.2.3
[0.2.2]: https://github.com/zarrs/ome_zarr_metadata/releases/tag/v0.2.2
[0.2.1]: https://github.com/zarrs/ome_zarr_metadata/releases/tag/v0.2.1
[0.2.0]: https://github.com/zarrs/ome_zarr_metadata/releases/tag/v0.2.0
[0.1.0]: https://github.com/zarrs/ome_zarr_metadata/releases/tag/v0.1.0

[@clbarnes]: https://github.com/clbarnes
