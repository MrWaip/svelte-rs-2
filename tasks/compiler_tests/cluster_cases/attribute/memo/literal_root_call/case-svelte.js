import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	$.init();
	var div = root();
	$.set_attribute(div, "title", $.untrack(() => "a".concat("b")));
	$.append($$anchor, div);
	$.pop();
}
