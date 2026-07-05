import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let bar = false;
	$$renderer.push(`<p>${$.escape((bar, false))}</p>`);
}
