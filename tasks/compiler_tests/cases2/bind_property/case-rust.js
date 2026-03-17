import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/> <details></details>`, 1);
export default function App($$anchor) {
	let indeterminate = $.state(false);
	let open = $.state(true);
	var fragment = root();
	var input = $.first_child(fragment);
	var details = $.sibling(input, 2);
	$.bind_property("indeterminate", "change", input, ($$value) => $.set(indeterminate, $$value), () => $.get(indeterminate));
	$.bind_property("open", "toggle", details, ($$value) => $.set(open, $$value), () => $.get(open));
	$.append($$anchor, fragment);
}
