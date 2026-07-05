import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.add_locations($.from_html(`<textarea></textarea>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.prop($$props, "value", 12, "");
	const bubbler = createBubbler();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var textarea = root();
	var event_handler = $.derived(() => bubbler("input"));
	$.remove_textarea_child(textarea);
	$.bind_value(textarea, function get() {
		return value();
	}, function set($$value) {
		value($$value);
	});
	$.event("input", textarea, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [6, 31], true);
	});
	$.append($$anchor, textarea);
	return $.pop($$exports);
}
