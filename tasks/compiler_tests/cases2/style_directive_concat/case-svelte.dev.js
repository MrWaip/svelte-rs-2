App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>Concat value</div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let shade = $.tag($.state("500"), "shade");
	$.set(shade, "600");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, { color: `red-${$.get(shade) ?? ""}` }));
	$.append($$anchor, div);
	return $.pop($$exports);
}
