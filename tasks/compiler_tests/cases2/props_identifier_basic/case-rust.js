import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const props = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $$props.name));
	$.append($$anchor, p);
	$.pop();
}
