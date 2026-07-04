App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let props = $.tag_proxy($.proxy({
		id: "a",
		style: "border-color: blue;"
	}), "props");
	let color = "red";
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({
		...props,
		[$.STYLE]: { color }
	}));
	$.append($$anchor, div);
	return $.pop($$exports);
}
