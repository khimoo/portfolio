# Khimoo Portfolio

Interactive portfolio website built with Rust and WebAssembly.

## Quick Start

```bash
# Development
just dev                 # Start dev server with file watchers
just dev-gh-pages       # Test with GitHub Pages path

# Build
just build              # Full production build
just process-data       # Process articles and images only

# Testing
just test               # Run all tests
just validate-links     # Check article links
```

## Project Structure

```
khimoo.io/
├── project.toml          # Central configuration
├── content/              # Articles and assets
├── khimoo-portfolio/     # Rust/WASM application
├── scripts/              # Build scripts
├── public/               # Deployment output
└── justfile              # Task runner
```

## Configuration

All settings are centralized in `project.toml` following DRY, KISS, and ETC principles:

```toml
[paths]
articles_dir = "content/articles"
images_dir = "content/assets/img"
data_dir = "khimoo-portfolio/data"

[deployment]
github_pages_path = "/portfolio-page/"
local_dev_path = "/"
```

## Development Workflow

1. **Edit content** in `content/articles/`
2. **File watcher** automatically rebuilds data
3. **Browser** auto-reloads with changes
4. **Deploy** with `just build`

## Architecture

- **Frontend**: Yew (Rust WebAssembly)
- **Physics**: Rapier2D for interactions
- **Content**: Markdown with frontmatter
- **Build**: Trunk + Just
- **Config**: Centralized TOML