App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function getTitle() {
		return "hello";
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(($0) => $.set_attribute(div, "title", $0), [() => getTitle()]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
