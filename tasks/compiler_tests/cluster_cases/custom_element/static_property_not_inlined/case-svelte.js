import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<my-element></my-element>`, 2);
export default function App($$anchor) {
	var my_element = root();
	$.set_custom_element_data(my_element, "text", "!");
	$.append($$anchor, my_element);
}
