App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let v = void 0;
	var $$exports = { ...$.legacy_api() };
	var input = root();
	input.defaultValue = "x";
	$.set_value(input, v);
	$.append($$anchor, input);
	return $.pop($$exports);
}
