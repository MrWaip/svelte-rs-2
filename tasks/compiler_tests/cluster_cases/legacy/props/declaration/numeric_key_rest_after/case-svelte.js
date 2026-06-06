import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		0
	]);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${$$props["0"] ?? ""} ${$$props.foo ?? ""}`));
	$.append($$anchor, text);
	$.pop();
}
