import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor) {
	let value = $.state("hello");
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.bind_value(textarea, () => $.get(value), ($$value) => $.set(value, $$value));
	$.append($$anchor, textarea);
}
