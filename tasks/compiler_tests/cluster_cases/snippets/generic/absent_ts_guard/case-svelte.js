import * as $ from "svelte/internal/client";
const plain = ($$anchor, val = $.noop) => {
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, val()));
	$.append($$anchor, text);
};
export default function App($$anchor) {}
