App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let spread = {};
	let v = void 0;
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.attribute_effect(input, () => ({
		defaultValue: "x",
		value: v,
		...spread
	}));
	$.append($$anchor, input);
	return $.pop($$exports);
}
