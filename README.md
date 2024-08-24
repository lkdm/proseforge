<div style="text-align:center">
  <p align="center">
    <img width="100" height="100" src="https://github.com/lkdm/proseforge/blob/main/apps/desktop/src-tauri/icons/128x128.png" alt="Proseforge logo">
  </p>
</div>

# Proseforge

Proseforge is an open-source cross-platform markdown editor written in Rust.

## Technologies

Proseforge uses Rust, React, Typescript, Tailwind, and Tauri.

## Monorepp structure

### Apps:

- `desktop` - A [Tauri](https://v2.tauri.app/) app.

### Core:

- `core`: The Rust core. Cross-platform core of the application that provides the main functionality.

### Interface:

- `interface`: The shared React component library.

### Packages:

- `markdown-editor`: A markdown editor component.

## Commands

- `make init` - Initialise the dev environment
- `make install` - Install dependencies
- `pnpm dev:desktop` - Start the desktop app
- `pnpm css` - Compile CSS and watch for changes
