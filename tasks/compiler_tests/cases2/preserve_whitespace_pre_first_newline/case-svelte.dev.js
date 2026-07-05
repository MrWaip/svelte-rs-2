App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<pre></pre>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = "hi";
	var $$exports = { ...$.legacy_api() };
	var pre = root();
	pre.textContent = "hi\n";
	$.append($$anchor, pre);
	return $.pop($$exports);
}
