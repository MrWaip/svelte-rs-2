import * as $ from "svelte/internal/client";
var root = $.from_html(`<div contenteditable=""></div> <div contenteditable=""></div> <div contenteditable=""></div>`, 1);
export default function App($$anchor) {
	let html = $.state("");
	let text = $.state("");
	let content = $.state("");
	var fragment = root();
	var div = $.first_child(fragment);
	var div_1 = $.sibling(div, 2);
	var div_2 = $.sibling(div_1, 2);
	$.bind_content_editable("innerHTML", div, () => $.get(html), ($$value) => $.set(html, $$value));
	$.bind_content_editable("innerText", div_1, () => $.get(text), ($$value) => $.set(text, $$value));
	$.bind_content_editable("textContent", div_2, () => $.get(content), ($$value) => $.set(content, $$value));
	$.append($$anchor, fragment);
}
