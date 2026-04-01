import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.from_html(`<registry>.Widget /></registry>`);
export default function App($$anchor) {
	const registry = { Widget };
	var registry_1 = root();
	$.append($$anchor, registry_1);
}
