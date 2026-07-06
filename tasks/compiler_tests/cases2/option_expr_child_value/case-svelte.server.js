import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = 1;
	$$renderer.push(`<select>`);
	$$renderer.option({}, value);
	$$renderer.push(`</select>`);
}
