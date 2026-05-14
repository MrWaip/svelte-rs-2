import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor, $$props) {
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, "", null, classes, { is_expanded: $$props.isExpanded }));
	$.append($$anchor, div);
}
