import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let props = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"id"
	]);
	const label = $.derived(() => $$props.label + "!");
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(label)));
	$.append($$anchor, p);
	$.pop();
}
