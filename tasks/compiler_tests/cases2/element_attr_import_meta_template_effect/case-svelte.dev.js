App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<a>x</a>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var a = root();
	$.template_effect(() => $.set_attribute(a, "href", import.meta.env.VITE_X));
	$.append($$anchor, a);
	return $.pop($$exports);
}
