App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>String value</div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let size = $.tag($.state("16px"), "size");
	$.set(size, "20px");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, {
		color: "red",
		"font-size": $.get(size)
	}));
	$.append($$anchor, div);
	return $.pop($$exports);
}
