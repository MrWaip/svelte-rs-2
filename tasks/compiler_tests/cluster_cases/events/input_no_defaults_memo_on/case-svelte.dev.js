import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.add_locations($.from_html(`<input type="text"/>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const bubbler = createBubbler();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var input = root();
	var event_handler = $.derived(() => bubbler("click"));
	$.event("click", input, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [5, 29], true);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
