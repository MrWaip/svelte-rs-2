import * as $ from "svelte/internal/client";
import { handler } from "./handlers.js";
var root_1 = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { get onerror() {
		return handler;
	} }, ($$anchor) => {
		var p = root_1();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
