import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button> <div></div>`, 1);
export default function App($$anchor) {
	let a = "red";
	let b = $.mutable_source("blue");
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, {
		color: a,
		background: $.get(b)
	}));
	$.event("click", button, () => $.set(b, "green"));
	$.append($$anchor, fragment);
}
