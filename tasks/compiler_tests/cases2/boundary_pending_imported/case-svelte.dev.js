App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { pending } from "./handlers.js";
var root = $.add_locations($.from_html(`<p>content</p>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { get pending() {
		return pending;
	} }, ($$anchor) => {
		var p = root();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
