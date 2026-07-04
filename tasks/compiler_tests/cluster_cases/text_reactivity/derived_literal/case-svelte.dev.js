App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1></h1>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const x = $.tag($.derived(() => 5), "x");
	var $$exports = { ...$.legacy_api() };
	var h1 = root();
	h1.textContent = $.get(x);
	$.append($$anchor, h1);
	return $.pop($$exports);
}
