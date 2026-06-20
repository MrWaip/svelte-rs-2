import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<my-element></my-element>`, 2);
export default function App($$anchor, $$props) {
	let obj = $.prop($$props, "obj", 8);
	var my_element = root();
	$.template_effect(() => $.set_custom_element_data(my_element, "camelCase", obj()));
	$.append($$anchor, my_element);
}
