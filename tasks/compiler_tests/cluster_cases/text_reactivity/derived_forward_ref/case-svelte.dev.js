App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1></h1>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const a = $.tag($.derived(() => $.get(b)), "a");
	const b = $.tag($.derived(() => 5), "b");
	var $$exports = { ...$.legacy_api() };
	var h1 = root();
	h1.textContent = $.get(a);
	$.append($$anchor, h1);
	return $.pop($$exports);
}
