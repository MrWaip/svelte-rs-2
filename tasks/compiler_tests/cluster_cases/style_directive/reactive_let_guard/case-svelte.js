import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button> <div></div>`, 1);
export default function App($$anchor) {
	let col = $.mutable_source("red");
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, { color: $.get(col) }));
	$.event("click", button, () => $.set(col, "blue"));
	$.append($$anchor, fragment);
}
