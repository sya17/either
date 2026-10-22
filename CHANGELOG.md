# Changelog

All notable changes to either are documented in this file. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project follows [Semantic Versioning](https://semver.org/) with the rules for `0.x` described in the [roadmap](docs/roadmap.md).

From the first release, this file is generated from Conventional Commit messages with `git-cliff`.

## [Unreleased]

either is in its foundation phase. Nothing has been released yet.

### Added

- Project structure: a Cargo workspace for the daemon, the CLI, and shared crates, and a place for the Quickshell shell.
- Quality gates: formatting, lints, tests, dependency and license checks, documentation checks, design token checks, SPDX headers, and a scope check, all run by `just check` and the git hooks.
- Design system: design tokens for the either Dusk and either Dawn themes, and the brand assets.
- Documentation: architecture, roadmap, development guides, and architecture decision records.
