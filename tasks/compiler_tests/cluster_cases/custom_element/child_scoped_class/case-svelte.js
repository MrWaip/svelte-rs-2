import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><my-element></my-element></div>`, 2);
export default function App($$anchor) {
	var div = root();
	var my_element = $.child(div);
	$.set_class(my_element, 1, "red svelte-p153w3");
	$.reset(div);
	$.append($$anchor, div);
}
