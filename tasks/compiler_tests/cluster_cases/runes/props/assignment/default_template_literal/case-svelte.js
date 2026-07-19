import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let label = $.prop($$props, "label", 19, () => `untitled`);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, label()));
	$.append($$anchor, text);
}
