import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const props = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	let title = $.derived(() => $$props.title);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $.get(title)));
	$.append($$anchor, text);
	$.pop();
}
