import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let onclick = $.prop($$props, "onclick", 8, undefined);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.delegated("click", div, function(...$$args) {
		$.apply(onclick, this, $$args, App, [4, 14]);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
$.delegate(["click"]);
