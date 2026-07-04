App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({ v: "x" }), "obj");
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.validate_binding("bind:value={obj.v}", [], () => obj, () => "v", 5, 7);
	$.bind_value(input, function get() {
		return obj.v;
	}, function set($$value) {
		obj.v = $$value;
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
