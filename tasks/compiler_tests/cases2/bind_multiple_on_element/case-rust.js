import * as $ from "svelte/internal/client";
var root = $.from_html(`<div contenteditable="">editable</div>`);
export default function App($$anchor) {
	let width = $.state(0);
	let content = $.state("");
	var div = root();
	$.bind_element_size(div, "clientWidth", ($$value) => $.set(width, $$value));
	$.bind_content_editable("innerHTML", div, () => $.get(content), ($$value) => $.set(content, $$value));
	$.append($$anchor, div);
}
