import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let s = "x";
	$$renderer.push(`<p>v x</p>`);
}
