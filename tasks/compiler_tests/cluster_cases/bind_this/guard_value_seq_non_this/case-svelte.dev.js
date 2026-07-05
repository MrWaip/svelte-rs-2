App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, () => $.get(x), (v) => $.set(x, v, true));
	$.append($$anchor, input);
	return $.pop($$exports);
}
