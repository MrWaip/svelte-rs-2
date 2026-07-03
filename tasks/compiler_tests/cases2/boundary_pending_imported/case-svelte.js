import * as $ from "svelte/internal/client";
import { pending } from "./handlers.js";
var root = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { get pending() {
		return pending;
	} }, ($$anchor) => {
		var p = root();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
