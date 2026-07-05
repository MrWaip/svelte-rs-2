App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
import { fade } from "svelte/transition";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag($.state(""), "value");
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.attach(input, () => tooltip);
	$.bind_value(input, function get() {
		return $.get(value);
	}, function set($$value) {
		$.set(value, $$value);
	});
	$.transition(3, input, () => fade);
	$.append($$anchor, input);
	return $.pop($$exports);
}
