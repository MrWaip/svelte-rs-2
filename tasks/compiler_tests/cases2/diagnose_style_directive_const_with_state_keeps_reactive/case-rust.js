import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <button>change</button>`, 1);
export default function App($$anchor) {
	const transform = "translateY(1px)";
	let color = $.state("red");
	var fragment = root();
	var div = $.first_child(fragment);
	let styles;
	var button = $.sibling(div, 2);
	$.template_effect(() => styles = $.set_style(div, "", styles, {
		transform,
		color: $.get(color)
	}));
	$.delegated("click", button, () => $.set(color, "blue"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
