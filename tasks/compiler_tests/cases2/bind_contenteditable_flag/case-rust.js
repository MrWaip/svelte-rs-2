import * as $ from "svelte/internal/client";
var root = $.from_html(`<div contenteditable="true"> </div>`);
export default function App($$anchor) {
	let html = $.state("");
	var div = root();
	var text = $.child(div);
	text.nodeValue = `text ${$.get(html) ?? ""}`;
	$.reset(div);
	$.bind_content_editable("innerHTML", div, () => $.get(html), ($$value) => $.set(html, $$value));
	$.append($$anchor, div);
}
