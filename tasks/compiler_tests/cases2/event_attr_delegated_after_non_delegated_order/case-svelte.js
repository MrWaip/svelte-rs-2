import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	function onFocus() {}
	function onKey() {}
	var div = root();
	$.event("focus", div, onFocus);
	$.delegated("keydown", div, onKey);
	$.append($$anchor, div);
}
$.delegate(["keydown"]);
