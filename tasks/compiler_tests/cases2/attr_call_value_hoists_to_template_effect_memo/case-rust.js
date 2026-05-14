import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	function getTitle() {
		return "hello";
	}
	var div = root();
	$.template_effect(($0) => $.set_attribute(div, "title", $0), [() => getTitle()]);
	$.append($$anchor, div);
}
