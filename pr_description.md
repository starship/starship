# Add jj_status module for Jujutsu VCS support

## Description

This PR implements a new `jj_status` module that displays Jujutsu repository status in the Starship prompt, similar to how `git_status` works for Git repositories.

## Changes Made

### ✅ Core Implementation
- **Module**: `src/modules/jj_status.rs` (176 lines) - Main module logic
- **Config**: `src/configs/jj_status.rs` (45 lines) - Configuration options
- **Registration**: Updated all required files for module registration

### ✅ Documentation  
- Added comprehensive `jj_status` section to `docs/config/README.md`
- Included all configuration options, variables, and examples
- Added `jj_status` to default prompt format

### ✅ Configuration
- Updated config schema: `cargo run --features config-schema`
- Added `jj_status` to `plain-text-symbols` preset
- Used simple git-style defaults for Jetpack compatibility

### ✅ Testing
- Added comprehensive unit tests covering:
  - No repository detection
  - Clean repository state
  - Staged, modified, untracked, and deleted changes
  - Custom configuration options
- All tests pass with `cargo test`

### ✅ Features Supported
- **Repository Detection**: Scans for `.jj` directories
- **Status Parsing**: Parses `jj status` command output
- **Status Indicators**: 
  - `+` for staged files
  - `!` for modified files  
  - `?` for untracked files
  - `✘` for deleted files
  - `=` for conflicted files
  - `»` for renamed files
- **Full Customization**: Users can customize all symbols and styling
- **Jetpack Compatible**: Simple defaults work with custom starship versions

## Usage Examples

### Default Configuration (Git-style symbols)
```toml
[jj_status]
format = "([\\[$all_status$ahead_behind\\]]($style) )"
style = "red bold"
staged = "+"
modified = "!"
untracked = "?"
deleted = "✘"
```

### Custom Fancy Symbols
```toml
[jj_status]
format = "([⎪$staged$modified$untracked$deleted⎥]($style))"
style = "bold italic blue"
staged = "[▪┤[$count](bold text)│](italic teal)"
modified = "[●◦](italic peach)"
untracked = "[◌◦](italic yellow)"
deleted = "[✕](italic red)"
```

## Testing

Run the tests:
```bash
cargo test jj_status
```

Test the module:
```bash
starship module jj_status
```

## Checklist

- [x] Add section to `docs/config/README.md` describing the module
- [x] Add variable to "Default Prompt Format" section  
- [x] Add options to presets in `docs/public/presets/toml`
- [x] Update config schema with `cargo run --features config-schema`
- [x] Create config structs in `src/configs/jj_status.rs`
- [x] Add entry in `PROMPT_ORDER` (`src/configs/starship_root.rs`)
- [x] Add entry in `FullConfig` (`src/configs/mod.rs`) 
- [x] Add entry in `ALL_MODULES` (`src/module.rs`)
- [x] Add `mod` declaration (`src/modules/mod.rs`)
- [x] Add entry in `handle()` (`src/modules/mod.rs`)
- [x] Add comprehensive unit tests
- [x] Test module functionality
- [x] Follow existing code patterns and conventions

## Related Issues

Closes #[GitHub issue number] - "Add JJ Status Module for Jujutsu VCS Support"

## Screenshots

In a JJ repository with changes:
```
[master] [⎪+!??✘⎥] ~/projects/my-jj-repo
```

This provides JJ users with the same status information they're used to seeing in their prompts, making Starship a complete solution for all major VCS systems.
