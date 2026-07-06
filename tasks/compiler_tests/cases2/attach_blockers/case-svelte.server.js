import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var data, handler;
	var $$promises = $$renderer.run([async () => data = await fetch("/api"), () => handler = data.handler]);
	$$renderer.push(`<div>hello</div>`);
}
