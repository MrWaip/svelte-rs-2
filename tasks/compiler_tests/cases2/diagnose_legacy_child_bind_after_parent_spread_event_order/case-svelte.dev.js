import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><input type="checkbox"/></div>`), App[$.FILENAME], [[
	7,
	0,
	[[8, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["checked"]);
	$.push($$props, false, App);
	let checked = $.prop($$props, "checked", 12, false);
	function k() {}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({ ...$$restProps }));
	var input = $.child(div);
	$.remove_input_defaults(input);
	$.reset(div);
	$.bind_checked(input, function get() {
		return checked();
	}, function set($$value) {
		checked($$value);
	});
	$.event("click", div, k);
	$.append($$anchor, div);
	return $.pop($$exports);
}
