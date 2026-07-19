import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let makeHandler = $.tag($.mutable_source(null), "makeHandler");
	$.set(makeHandler, () => () => console.log("x"));
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var event_handler = $.derived(() => $.get(makeHandler)());
	$.event("click", button, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [6, 18], true, true);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
