<div style="text-align:center">
  <p align="center">
    <img width="100" height="100" src="https://github.com/lkdm/proseforge/blob/main/apps/desktop/src-tauri/icons/128x128.png" alt="Proseforge logo">
  </p>
</div>

# Proseforge

Proseforge is an open-source cross-platform markdown editor written in Rust.

![Screenshot 2024-08-25 at 10 31 34 AM](https://github.com/user-attachments/assets/c486d959-28c6-407c-b1c1-9f09f5591cd4)

## Technologies


Proseforge uses Rust, React, Typescript, Tailwind, and Tauri.

## Monorepo structure

### Apps:

- `desktop` - A [Tauri](https://v2.tauri.app/) app.

### Core

The core module provides the fundamental functionality of the application, organised into major features.

- **Top-level directories**: Each directory under core represents a major feature of the application.
- **Feature subdirectories**: Each major feature directory contains subdirectories named after the main data types of that feature.
    - **TYPE.rs files**: Define the primary data types and validation logic.
    - **TYPE/models.rs**: Contains the data structures, including any associated types and validation rules.
    - **TYPE/ports.rs**: Defines the interfaces and abstractions for interactions related to the feature.
    - **TYPE/services.rs**: Implements the business logic and service layer for managing and manipulating the data.

### Interface:

- `interface`: The shared React component library.

### Packages:

- `markdown-editor`: A markdown editor component.

## Commands

- `make init` - Initialise the dev environment
- `make install` - Install dependencies
- `pnpm dev:desktop` - Start the desktop app
- `pnpm css` - Compile CSS and watch for changes
- `pnpm run desktop tauri build` - Build the desktop app
