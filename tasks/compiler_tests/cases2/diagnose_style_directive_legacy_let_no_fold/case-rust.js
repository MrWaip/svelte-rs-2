import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <button>bump</button>`, 1);
export default function App($$anchor) {
	let x = 1;
	var fragment = root();
	var div = $.first_child(fragment);
	let styles;
	var button = $.sibling(div, 2);
	$.template_effect(() => styles = $.set_style(div, "", styles, { opacity: x }));
	$.event("click", button, () => x = 2);
	$.append($$anchor, fragment);
}
