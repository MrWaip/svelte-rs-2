import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	$$renderer.push(`<div${$.attr("title", count)}></div> <button>go</button>`);
}
