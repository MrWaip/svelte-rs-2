import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { x: null };
	let src = {};
	$$renderer.push(`<button>go</button>`);
}
