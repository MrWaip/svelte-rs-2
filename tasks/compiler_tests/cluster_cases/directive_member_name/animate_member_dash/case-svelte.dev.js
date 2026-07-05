App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const fn = (node, options) => ({});
	let a = { b: { "c-d": fn } };
	let directive = $.tag($.derived(() => a), "directive");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node_1, 24, () => [], (i) => i, ($$anchor, i) => {
		var div = root();
		$.animation(div, () => $.get(directive).b["c-d"], null);
		$.append($$anchor, div);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
