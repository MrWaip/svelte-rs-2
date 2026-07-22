import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <div></div> <div></div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var div = $.first_child(fragment);
	$.set_attribute(div, "aria-level", `abc`);
	var div_1 = $.sibling(div, 2);
	$.set_attribute(div_1, "aria-level", "false");
	var div_2 = $.sibling(div_1, 2);
	$.set_attribute(div_2, "title", `x`);
	$.append($$anchor, fragment);
}
