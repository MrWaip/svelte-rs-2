import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.prop($$props, "value", 8);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.template_effect(() => $.set_attribute(input, "data-x", `a${value() ?? ""}`));
	$.append($$anchor, input);
	return $.pop($$exports);
}
