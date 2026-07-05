App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let content = "<b>hello</b>";
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.html(div, () => content, true);
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
