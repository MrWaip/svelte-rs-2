import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	var div = root();
	let classes;
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(($0) => {
		classes = $.set_class(div, 1, "", null, classes, $0);
		$.set_text(text, $$props.name);
	}, [() => ({
		x: $$props.a,
		y: Boolean($$props.onClick)
	})]);
	$.append($$anchor, div);
}
