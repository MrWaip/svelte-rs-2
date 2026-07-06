import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let focused = false;
	$$renderer.push(`<button${$.attr("focused", focused)}></button>`);
}
