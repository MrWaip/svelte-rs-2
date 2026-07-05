import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let checked = $.prop($$props, "checked", 12, false);
	const bubbler = createBubbler();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var input = root();
	var event_handler = $.derived(() => bubbler("change"));
	$.remove_input_defaults(input);
	$.bind_checked(input, function get() {
		return checked();
	}, function set($$value) {
		checked($$value);
	});
	$.event("change", input, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [6, 47], true);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
