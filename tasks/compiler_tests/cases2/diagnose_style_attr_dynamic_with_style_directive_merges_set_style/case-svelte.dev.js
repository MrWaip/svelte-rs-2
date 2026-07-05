App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let position = $.prop($$props, "position", 3, "static"), pb = $.prop($$props, "pb", 3, "");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, `position: ${position() ?? ""}`, styles, { "--x": pb() }));
	$.append($$anchor, div);
	return $.pop($$exports);
}
