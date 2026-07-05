import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let checked = $.prop($$props, "checked", 12, false);
	const bubbler = createBubbler();
	const u = "http://x";
	var $$exports = { ...$.legacy_api() };
	$.init();
	var input = root();
	var event_handler = $.derived(() => bubbler("change"));
	$.remove_input_defaults(input);
	$.set_attribute(input, "formaction", u);
	$.bind_checked(input, function get() {
		return checked();
	}, function set($$value) {
		checked($$value);
	});
	$.event("change", input, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [7, 62], true);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
