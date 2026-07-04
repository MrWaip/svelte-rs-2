App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>Click</button>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function getHandler() {
		return () => console.log("clicked");
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var event_handler = $.derived(getHandler);
	$.delegated("click", button, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [7, 17], true, true);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
