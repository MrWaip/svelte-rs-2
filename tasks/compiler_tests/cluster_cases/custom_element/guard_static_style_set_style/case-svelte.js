import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<my-el></my-el>`, 2);
export default function App($$anchor) {
	var my_el = root();
	$.set_style(my_el, "color: red");
	$.append($$anchor, my_el);
}
