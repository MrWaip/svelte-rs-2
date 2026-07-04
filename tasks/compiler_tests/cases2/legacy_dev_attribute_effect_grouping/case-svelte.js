import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>bump</button> <div></div>`, 1);
export default function App($$anchor) {
	let title = $.mutable_source("t");
	let counter = $.mutable_source(0);
	let flag = $.mutable_source("x");
	function bump() {
		$.set(title, $.get(title) + "!");
		$.set(counter, $.get(counter) + 1);
		$.set(flag, $.get(flag) + "!");
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	$.template_effect(() => {
		$.set_attribute(div, "title", $.get(title));
		$.set_attribute(div, "data-counter", $.get(counter));
		$.set_attribute(div, "data-flag", $.get(flag));
	});
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
