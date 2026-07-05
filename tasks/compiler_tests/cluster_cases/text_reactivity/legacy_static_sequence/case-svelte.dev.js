import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<pre></pre>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let y = $.prop($$props, "y", 8);
	var $$exports = { ...$.legacy_api() };
	var pre = root();
	pre.textContent = (1, "");
	$.append($$anchor, pre);
	return $.pop($$exports);
}
