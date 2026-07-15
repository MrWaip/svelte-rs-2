import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let cache = {};
	function fill(items) {
		items.forEach((item) => $.assign(cache, item.id, "=", item, "(unknown):6:24"));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.event("click", button, function click() {
		return fill([]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
