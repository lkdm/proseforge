install:
	pnpm recursive install

run:
	pnpm run dev:desktop

css:
	pnpm --filter @pc/ui css-watch
