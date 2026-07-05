import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let active = true;
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.set_class(div, 1, "", null, {}, { active });
	$.append($$anchor, div);
	return $.pop($$exports);
}
