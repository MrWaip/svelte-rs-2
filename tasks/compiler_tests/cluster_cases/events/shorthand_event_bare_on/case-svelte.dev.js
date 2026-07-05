import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let on = $.prop($$props, "on", 8, undefined);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.event("", div, function(...$$args) {
		$.apply(on, this, $$args, App, [4, 6]);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
