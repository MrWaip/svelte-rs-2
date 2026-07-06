import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = void 0;
	value = 42;
	$$renderer.push(`<p>${$.escape(value)}</p>`);
}
