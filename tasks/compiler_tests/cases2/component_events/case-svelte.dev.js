App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function done() {}
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Widget($$anchor, { $$events: { done } }), "component", App, 7, 0, { componentTag: "Widget" });
	return $.pop($$exports);
}
