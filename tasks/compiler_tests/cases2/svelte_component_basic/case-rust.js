import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.from_html(`<svelte:component></svelte:component>`);
export default function App($$anchor) {
	let current = A;
	var svelte:component = root();
	$.set_attribute(svelte:component, "this", current);
	$.set_attribute(svelte:component, "answer", 42);
	$.append($$anchor, svelte:component);
}
