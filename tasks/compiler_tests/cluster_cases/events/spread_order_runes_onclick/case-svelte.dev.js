App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function f() {
		return () => {};
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, ($0) => ({
		...$$props.rest,
		onclick: $0
	}), [() => f()]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
