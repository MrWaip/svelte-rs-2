import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rest = $.prop($$props, "rest", 24, () => ({}));
	function onClick() {}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({ ...rest() }));
	$.event("click", div, onClick);
	$.append($$anchor, div);
	return $.pop($$exports);
}
