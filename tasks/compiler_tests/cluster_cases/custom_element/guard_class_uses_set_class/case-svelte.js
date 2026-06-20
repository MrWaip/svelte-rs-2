import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<my-element></my-element>`, 2);
export default function App($$anchor, $$props) {
	let cls = $.prop($$props, "cls", 8);
	var my_element = root();
	$.template_effect(() => $.set_class(my_element, 1, $.clsx(cls())));
	$.append($$anchor, my_element);
}
