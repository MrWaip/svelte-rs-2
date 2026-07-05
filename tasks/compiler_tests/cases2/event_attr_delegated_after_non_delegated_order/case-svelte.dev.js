App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function onFocus() {}
	function onKey() {}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.event("focus", div, onFocus);
	$.delegated("keydown", div, onKey);
	$.append($$anchor, div);
	return $.pop($$exports);
}
$.delegate(["keydown"]);
