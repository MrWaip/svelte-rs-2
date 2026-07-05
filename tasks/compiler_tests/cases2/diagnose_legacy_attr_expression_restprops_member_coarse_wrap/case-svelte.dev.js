import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["name"]);
	$.push($$props, false, App);
	let name = $.prop($$props, "name", 8, "n");
	var $$exports = { ...$.legacy_api() };
	$.init();
	var div = root();
	$.template_effect(() => $.set_attribute(div, "id", ($.deep_read_state($$restProps), $.deep_read_state(name()), $.untrack(() => $$restProps.id || name()))));
	$.append($$anchor, div);
	return $.pop($$exports);
}
