import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	const props = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const a = $.derived(() => $$props.a), b = $.derived(() => $$props.b);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""},${$.get(b) ?? ""}`));
	$.append($$anchor, p);
}
