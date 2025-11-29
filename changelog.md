# Changelog

## v0.1.0

- Initial release

### v0.1.1

- Refactored functions
- Made code more readable

### v0.1.2

- Added a more simplified command not found message
- Added minimal configuration support
  - Located in `$HOME/.config/minsh/config.toml`

### v0.1.3

- Minsh no longer wraps shells, it now executes commands directly
  - Removed config:
    `[commands]`, `shell`
