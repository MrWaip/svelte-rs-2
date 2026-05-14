import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let onCb = $.prop($$props, "onCb", 8, () => {});
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, onCb()));
	$.append($$anchor, text);
}
