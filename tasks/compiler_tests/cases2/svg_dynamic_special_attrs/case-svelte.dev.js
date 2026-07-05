App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<rect></rect>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = 1;
	let disabled = false;
	var $$exports = { ...$.legacy_api() };
	var rect = root();
	$.set_value(rect, value);
	rect.disabled = disabled;
	$.append($$anchor, rect);
	return $.pop($$exports);
}
