import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<my-element></my-element>`, 2);
export default function App($$anchor, $$props) {
	let count = $.prop($$props, "count", 8);
	var my_element = root();
	$.template_effect(() => $.set_custom_element_data(my_element, "value", count()));
	$.append($$anchor, my_element);
}
