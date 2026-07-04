import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let entries = $.prop($$props, "entries", 12);
	function put(item, value) {
		entries(entries()[item.id] = value, true);
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	$.event("click", button, function click() {
		return put({ id: "a" }, 1);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
