App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div contenteditable="">editable</div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let width = $.tag($.state(0), "width");
	let content = $.tag($.state(""), "content");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_element_size(div, "clientWidth", function set($$value) {
		$.set(width, $$value);
	});
	$.bind_content_editable("innerHTML", div, function get() {
		return $.get(content);
	}, function set($$value) {
		$.set(content, $$value);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
