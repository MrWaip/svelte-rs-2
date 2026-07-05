App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let enabled = true;
	let handler = $.tag($.derived(() => enabled ? (node) => {} : null), "handler");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, {
		[$.attachment()]: ($$node) => $.get(handler)($$node),
		prop: "value"
	}), "component", App, 8, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
