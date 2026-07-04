App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>Click</button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	function getHandler() {
		return () => $.update(count);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var event_handler = $.derived(getHandler);
	$.delegated("click", button, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [9, 17], true, true);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
