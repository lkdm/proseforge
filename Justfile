
install:
	pnpm recursive install
	cargo install --path ./apps/desktop/src-tauri

watch css:
    @echo "Watching CSS changes"
    pnpm --filter @md/interface css-watch

dev desktop:
    @echo "Running Desktop in development"
    pushd apps/desktop/src-tauri && pnpm tauri dev
