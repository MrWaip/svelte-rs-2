import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <button>go</button>`, 1);
export default function App($$anchor) {
	let count = $.mutable_source(0);
	function bump() {
		$.set(count, $.get(count) + 1);
	}
	var fragment = root();
	var div = $.first_child(fragment);
	$.set_attribute(div, "title", [() => $.get(count)]);
	var button = $.sibling(div, 2);
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
