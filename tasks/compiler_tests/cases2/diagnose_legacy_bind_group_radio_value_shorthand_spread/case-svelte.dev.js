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
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["value", "group"]);
	$.push($$props, false, App);
	const binding_group = [];
	let value = $.prop($$props, "value", 8);
	let group = $.prop($$props, "group", 12);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.attribute_effect(input, () => ({
		type: "radio",
		value: value(),
		...$$restProps
	}), void 0, void 0, void 0, void 0, true);
	$.bind_group(binding_group, [], input, () => {
		value();
		return group();
	}, function set($$value) {
		group($$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
