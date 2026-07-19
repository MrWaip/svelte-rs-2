App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let make = (name) => ({ handler: () => console.log(...$.log_if_contains_state("log", name)) });
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var event_handler = $.derived(() => make("Tama").handler);
	$.delegated("click", button, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [5, 17]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
