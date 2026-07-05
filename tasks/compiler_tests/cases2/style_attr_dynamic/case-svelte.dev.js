App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let color = $.tag($.state("red"), "color");
	function toggle() {
		$.set(color, "blue");
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(() => $.set_style(div, `color: ${$.get(color) ?? ""}; font-size: 14px`));
	$.append($$anchor, div);
	return $.pop($$exports);
}
