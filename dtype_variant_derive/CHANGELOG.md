# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.13](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.12...dtype_variant_derive-v0.0.13) - 2025-06-17

### Other

- `cargo fmt`
- Simplify logic and code reuse.
- Prefix generated struct with Enum names
- Added support for struct variants, using generate structs

## [0.0.12](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.11...dtype_variant_derive-v0.0.12) - 2025-04-28

### Other

- Change grouped_matcher syntax to use `|` instead.

## [0.0.11](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.10...dtype_variant_derive-v0.0.11) - 2025-04-27

### Other

- Allow multiple grouped matcher generation

## [0.0.10](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.9...dtype_variant_derive-v0.0.10) - 2025-04-18

### Other

- Merge pull request #19 from Swoorup/sj-fix-crate-path
- update dtype attribute syntax to use tokens_path for clarity

## [0.0.9](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.8...dtype_variant_derive-v0.0.9) - 2025-04-18

### Other

- simplify token path generation in matcher methods

## [0.0.8](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.7...dtype_variant_derive-v0.0.8) - 2025-04-14

### Other

- Merge pull request #15 from Swoorup/sj_match_category
- Make source type optional

## [0.0.7](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.6...dtype_variant_derive-v0.0.7) - 2025-04-14

### Other

- Do not skip from impls for unit variants
- Added `grouped_matcher` feature and allow skipping from impls

## [0.0.6](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.5...dtype_variant_derive-v0.0.6) - 2025-04-12

### Other

- Merge pull request #11 from Swoorup/sj-dest-constraint-pattern
- Fix generics, and allow generic pattern match

## [0.0.5](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.4...dtype_variant_derive-v0.0.5) - 2025-04-12

### Other

- Merge pull request #9 from Swoorup/sj-dest-constraint-pattern
- Enhance enum variant handling and matcher generation with constraints

## [0.0.4](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.3...dtype_variant_derive-v0.0.4) - 2025-04-12

### Other

- Update readme for better example

## [0.0.3](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.2...dtype_variant_derive-v0.0.3) - 2025-04-12

### Added

- matcher can target dest enum and enhance DPrimType example with create_chunk method

### Other

- Omit generic binding if include_inner is false

## [0.0.2](https://github.com/Swoorup/dtype_variant/compare/dtype_variant_derive-v0.0.1...dtype_variant_derive-v0.0.2) - 2025-04-12

### Added

- Add dtype_variant_path function and update derive implementation

### Other

- Merge pull request #5 from Swoorup/sj-more-patterns
- Added licence and readme symlink to crates

## [0.0.1](https://github.com/Swoorup/dtype_variant/releases/tag/dtype_variant_derive-v0.0.1) - 2025-04-12

### Added

- Initial implementation of dtype_variant
