App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div contenteditable="true"> </div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let html = $.tag($.state(""), "html");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var text = $.child(div);
	text.nodeValue = `text ${$.get(html) ?? ""}`;
	$.reset(div);
	$.bind_content_editable("innerHTML", div, function get() {
		return $.get(html);
	}, function set($$value) {
		$.set(html, $$value);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
