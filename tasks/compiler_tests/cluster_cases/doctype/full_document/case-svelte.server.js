import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!doctype html=""/> <html lang="en"><head><meta charset="utf-8"/> <title>Svelte App</title></head> <body><div>Hello World</div></body></html>`);
}
