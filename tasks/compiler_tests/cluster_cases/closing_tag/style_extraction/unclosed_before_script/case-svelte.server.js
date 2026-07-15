import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 1;
	$$renderer.push(`<div><span>x</span></div>`);
}
