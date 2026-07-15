App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export const svelte4space = "svelte4space";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
