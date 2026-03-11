import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"x"
	]);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $$props.x));
	$.append($$anchor, p);
}
