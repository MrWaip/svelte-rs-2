App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.prop($$props, "x", 3, "foo");
	const cond = $.strict_equals(x(), "foo");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(() => $.set_class(div, 1, `a ${cond && "b"}`));
	$.append($$anchor, div);
	return $.pop($$exports);
}
