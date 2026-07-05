App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export const API_URL = "/api";
export function formatDate(d) {
	return d.toISOString();
}
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = "hello";
	var $$exports = { ...$.legacy_api() };
	var p = root();
	p.textContent = "hello";
	$.append($$anchor, p);
	return $.pop($$exports);
}
