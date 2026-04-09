App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	function getHandler() {
		return () => $.update(count);
	}
	var $$exports = { ...$.legacy_api() };
	var event_handler = $.derived(getHandler);
	$.add_svelte_meta(() => Widget($$anchor, { $$events: { done(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [11, 17], true, true);
	} } }), "component", App, 11, 0, { componentTag: "Widget" });
	return $.pop($$exports);
}
