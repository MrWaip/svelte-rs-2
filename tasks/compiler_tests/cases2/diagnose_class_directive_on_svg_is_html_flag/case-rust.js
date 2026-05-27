import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><path d="M0 0"></path></svg>`);
export default function App($$anchor, $$props) {
	let active = $.prop($$props, "active", 3, false);
	var svg = root();
	let classes;
	$.template_effect(() => classes = $.set_class(svg, 1, "icon", null, classes, { active: active() }));
	$.append($$anchor, svg);
}
