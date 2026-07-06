import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div data:foo="bar">hello</div>`);
}
