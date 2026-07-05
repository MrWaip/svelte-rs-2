App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const H = 8;
	let w = $.tag($.state(0), "w");
	setTimeout(() => {
		$.set(w, 10);
	});
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, {
		height: "8px",
		width: `${$.get(w) ?? ""}px`
	}));
	$.append($$anchor, div);
	return $.pop($$exports);
}
