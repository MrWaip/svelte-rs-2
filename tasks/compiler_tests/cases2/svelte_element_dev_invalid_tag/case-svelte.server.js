import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "#text";
	$.element($$renderer, tag);
}
