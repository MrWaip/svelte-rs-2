App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let opacity = $.prop($$props, "opacity", 3, .5);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attach(div, () => (node) => $.assign(node.style, "opacity", "=", opacity(), "(unknown):5:24"));
	$.append($$anchor, div);
	return $.pop($$exports);
}
