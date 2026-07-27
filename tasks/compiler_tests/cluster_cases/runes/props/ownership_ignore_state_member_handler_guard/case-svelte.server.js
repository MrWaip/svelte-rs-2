import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let s = { x: 0 };
	$$renderer.push(`<button></button>`);
}
