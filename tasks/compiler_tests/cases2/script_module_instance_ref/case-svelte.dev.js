App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const BASE = "https://example.com";
var root = $.add_locations($.from_html(`<a>Link</a>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let path = "/home";
	let url = $.tag($.derived(() => BASE + path), "url");
	var $$exports = { ...$.legacy_api() };
	var a = root();
	$.set_attribute(a, "href", $.get(url));
	$.append($$anchor, a);
	return $.pop($$exports);
}
