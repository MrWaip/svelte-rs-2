import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let offsetX = $.prop($$props, "offsetX", 8, "");
	let paddingX = $.prop($$props, "paddingX", 24, offsetX);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, paddingX()));
	$.append($$anchor, div);
	return $.pop($$exports);
}
