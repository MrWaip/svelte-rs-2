import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["name", "checked"]);
	$.push($$props, false, App);
	let name = $.prop($$props, "name", 8, "");
	let checked = $.prop($$props, "checked", 12, false);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var input = root();
	$.attribute_effect(input, () => ({
		type: "checkbox",
		id: ($.deep_read_state($$restProps), $.deep_read_state(name()), $.untrack(() => $$restProps.id || name())),
		...$$restProps
	}), void 0, void 0, void 0, void 0, true);
	$.bind_checked(input, function get() {
		return checked();
	}, function set($$value) {
		checked($$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
