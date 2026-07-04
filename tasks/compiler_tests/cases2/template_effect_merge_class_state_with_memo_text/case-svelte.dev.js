App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><span> </span></div>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 2]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.prop($$props, "x", 3, 0);
	function fmt(n) {
		return String(n);
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	var span = $.child(div);
	var text = $.child(span, true);
	$.reset(span);
	$.reset(div);
	$.template_effect(($0) => {
		classes = $.set_class(div, 1, "", null, classes, { active: $.strict_equals(x(), 0) });
		$.set_text(text, $0);
	}, [() => fmt(x())]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
