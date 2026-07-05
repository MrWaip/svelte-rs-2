import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div data-count="value: 1231"></div>`);
}
