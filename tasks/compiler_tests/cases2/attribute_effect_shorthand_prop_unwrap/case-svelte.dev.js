App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"width",
	"alt"
]);
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let width = $.prop($$props, "width", 3, 1), rest = $.rest_props($$props, rest_excludes, "rest");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({
		width: width(),
		alt: $$props.alt,
		...rest
	}));
	$.append($$anchor, div);
	return $.pop($$exports);
}
