App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = $.tag($.state(0), "n");
	function makeHandler() {
		return () => $.update(n);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var event_handler = $.derived(makeHandler);
	$.delegated("click", button, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [8, 17], true, true);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
