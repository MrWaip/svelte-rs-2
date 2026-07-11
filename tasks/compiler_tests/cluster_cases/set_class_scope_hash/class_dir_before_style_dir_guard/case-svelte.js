import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>a</div>`);
export default function App($$anchor, $$props) {
	var div = root();
	let classes;
	$.set_style(div, "", {}, { color: "red" });
	$.template_effect(() => classes = $.set_class(div, 1, "svelte-lvqw8l", null, classes, { active: $$props.on }));
	$.append($$anchor, div);
}
