import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let s = $.prop($$props, "s", 8);
	let attributes = $.prop($$props, "attributes", 24, () => ({}));
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({
		style: s(),
		...attributes()
	}));
	$.append($$anchor, div);
	return $.pop($$exports);
}
