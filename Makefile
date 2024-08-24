install:
	pnpm recursive install
	cargo install --path ./core
	cargo install --path ./apps/desktop/src-tauri

run:
	pnpm run dev:desktop

css:
	pnpm --filter @md/interface css-watch
