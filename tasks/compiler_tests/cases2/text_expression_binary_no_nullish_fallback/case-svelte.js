import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let a = $.prop($$props, "a", 8, 1);
	let b = $.prop($$props, "b", 8, 2);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${a() ?? ""} ${a() + b()}`));
	$.append($$anchor, text);
}
