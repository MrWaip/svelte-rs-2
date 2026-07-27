import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <button>go</button>`, 1), App[$.FILENAME], [[8, 0], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = $.tag($.mutable_source(0), "count");
	function bump() {
		$.set(count, $.get(count) + 1);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	$.set_attribute(div, "title", [() => $.get(count)]);
	var button = $.sibling(div, 2);
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
