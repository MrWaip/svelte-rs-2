import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let size = 1;
	$.element($$renderer, `h${size}`);
}
