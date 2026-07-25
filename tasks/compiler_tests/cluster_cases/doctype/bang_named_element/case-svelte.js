import * as $ from "svelte/internal/client";
var root = $.from_html(`<!foo>bar</!foo>`);
export default function App($$anchor) {
	var _foo = root();
	$.append($$anchor, _foo);
}
