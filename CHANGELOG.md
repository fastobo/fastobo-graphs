# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
[Unreleased]: https://github.com/fastobo/fastobo-graphs/compare/v0.4.4...HEAD

## [v0.4.4] - 2022-01-20
[v0.4.4]: https://github.com/fastobo/fastobo-graphs/compare/v0.4.3...v0.4.4
### Changed
- Bumped `fastobo` to `v0.14.0`.
- Implement extraction of `Ontology` and `DataVersion` header clauses from OBO graphs.

## [v0.4.3] - 2021-02-18
[v0.4.3]: https://github.com/fastobo/fastobo-graphs/compare/v0.4.2...v0.4.3
### Changed
- Bumped `fastobo` to `v0.13.0`.
- Replaced Travis-CI with GitHub Actions for CI/CD.

## [v0.4.2] - 2020-11-17
[v0.4.2]: https://github.com/fastobo/fastobo-graphs/compare/v0.4.1...v0.4.2
### Changed
- Bumped `fastobo` to `v0.12.0`.
- Replaced `err-derive` dependency with `thiserror` crate.

## [v0.4.1] - 2020-08-30
[v0.4.1]: https://github.com/fastobo/fastobo-graphs/compare/v0.4.0...v0.4.1
### Changed
- Bumped `fastobo` to `v0.11.0`.

## [v0.4.0] - 2020-07-27
[v0.4.0]: https://github.com/fastobo/fastobo-graphs/compare/v0.3.0...v0.4.0
### Changed
- Bumped `fastobo` to `v0.10.0`.

## [v0.3.0] - 2020-06-14
[v0.3.0]: https://github.com/fastobo/fastobo-graphs/compare/v0.2.0...v0.3.0
### Changed
- Bumped `fastobo` version to `v0.9.0`.

## [v0.2.0] - 2020-01-23
[v0.2.0]: https://github.com/fastobo/fastobo-graphs/compare/v0.1.2...v0.2.0
### Changed
- Bumped `fastobo` version to `v0.8.0`.

## [v0.1.2] - 2019-08-27
[v0.1.2]: https://github.com/fastobo/fastobo-graphs/compare/v0.1.1...v0.1.2
### Fixed
- Fixed `to_string` being used instead of `into_string` in some `IntoGraphCtx` impl.
### Added
- Added BOSC 2019 poster reference to `README.md`.

## [v0.1.1] - 2019-08-08
[v0.1.1]: https://github.com/fastobo/fastobo-graphs/compare/v0.1.0...v0.1.1
### Fixed
- `fastobo_graphs::to_file` not creating a file with `File::create`.
### Added
- Improved documentation in and added example in `README.md`.

## [v0.1.0] - 2019-08-06
[v0.1.0]: https://github.com/fastobo/fastobo-graphs/compare/a3d5dff...v0.1.0
Initial release.
