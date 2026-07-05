App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div contenteditable=""></div> <div contenteditable=""></div> <div contenteditable=""></div>`, 1), App[$.FILENAME], [
	[7, 0],
	[9, 0],
	[11, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let html = $.tag($.state(""), "html");
	let text = $.tag($.state(""), "text");
	let content = $.tag($.state(""), "content");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	var div_1 = $.sibling(div, 2);
	var div_2 = $.sibling(div_1, 2);
	$.bind_content_editable("innerHTML", div, function get() {
		return $.get(html);
	}, function set($$value) {
		$.set(html, $$value);
	});
	$.bind_content_editable("innerText", div_1, function get() {
		return $.get(text);
	}, function set($$value) {
		$.set(text, $$value);
	});
	$.bind_content_editable("textContent", div_2, function get() {
		return $.get(content);
	}, function set($$value) {
		$.set(content, $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
