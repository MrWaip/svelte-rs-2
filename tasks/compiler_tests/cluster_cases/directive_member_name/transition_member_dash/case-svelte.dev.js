App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const fn = (node, options) => ({});
	let a = { b: { "c-d": fn } };
	let directive = $.tag($.derived(() => a), "directive");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.transition(3, div, () => $.get(directive).b["c-d"]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
