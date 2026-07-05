import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let n = $.tag($.mutable_source(0), "n");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.event("click", button, function click() {
		return $.update(n);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
