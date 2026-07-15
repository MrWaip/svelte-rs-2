App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = 0;
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.event("click", button, $.preventDefault((e) => n++));
	$.append($$anchor, button);
	return $.pop($$exports);
}
