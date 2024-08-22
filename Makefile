install:
	pnpm recursive install

run:
	pnpm run dev:desktop

css:
	pnpm --filter @md/interface css-watch
