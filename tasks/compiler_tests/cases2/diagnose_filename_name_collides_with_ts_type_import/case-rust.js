import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function DepositMethod($$anchor, $$props) {
	$.push($$props, true);
	let props = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $$props.title));
	$.append($$anchor, p);
	$.pop();
}
