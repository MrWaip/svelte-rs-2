import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button is="my-button"></button>`, 2);
export default function App($$anchor, $$props) {
	let obj = $.prop($$props, "obj", 8);
	var button = root();
	$.template_effect(() => $.set_custom_element_data(button, "foo", obj()));
	$.append($$anchor, button);
}
