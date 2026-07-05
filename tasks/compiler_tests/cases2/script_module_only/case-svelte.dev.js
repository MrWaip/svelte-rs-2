App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export const VERSION = "1.0.0";
var root = $.add_locations($.from_html(`<p>Static content</p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	$.append($$anchor, p);
	return $.pop($$exports);
}
