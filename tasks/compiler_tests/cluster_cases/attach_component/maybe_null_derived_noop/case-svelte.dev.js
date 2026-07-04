App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let handler = $.tag($.derived(() => $$props.maybe ? (node) => {} : null), "handler");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, { [$.attachment()]: ($$node) => ($.get(handler) || $.noop)($$node) }), "component", App, 9, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
