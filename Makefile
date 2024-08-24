install:
	pnpm recursive install
	cargo install --path ./core
	cargo install --path ./apps/desktop/src-tauri
