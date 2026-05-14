import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><span> </span></div>`);
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "x", 3, 0);
	function fmt(n) {
		return String(n);
	}
	var div = root();
	let classes;
	var span = $.child(div);
	var text = $.child(span, true);
	$.reset(span);
	$.reset(div);
	$.template_effect(($0) => {
		classes = $.set_class(div, 1, "", null, classes, { active: x() === 0 });
		$.set_text(text, $0);
	}, [() => fmt(x())]);
	$.append($$anchor, div);
}
