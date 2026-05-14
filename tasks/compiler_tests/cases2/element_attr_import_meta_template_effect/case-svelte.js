import * as $ from "svelte/internal/client";
var root = $.from_html(`<a>x</a>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var a = root();
	$.template_effect(() => $.set_attribute(a, "href", import.meta.env.VITE_X));
	$.append($$anchor, a);
	$.pop();
}
