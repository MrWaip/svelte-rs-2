App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div title="hockey" visible=""></div>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.set_attribute(div, "expression", name);
	$.set_attribute(div, "description", description);
	$.set_attribute(div, "index", `number: ${idx ?? ""}`);
	$.append($$anchor, div);
	return $.pop($$exports);
}
