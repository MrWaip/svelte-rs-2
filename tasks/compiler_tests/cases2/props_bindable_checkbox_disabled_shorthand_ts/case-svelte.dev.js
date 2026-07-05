App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let checked = $.prop($$props, "checked", 15, false), disabled = $.prop($$props, "disabled", 3, false);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.template_effect(() => input.disabled = disabled());
	$.bind_checked(input, function get() {
		return checked();
	}, function set($$value) {
		checked($$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
