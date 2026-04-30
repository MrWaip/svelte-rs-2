import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let obj = $.prop($$props, "obj", 23, () => ({ x: 0 }));
	obj().x;
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, obj().x++));
	$.append($$anchor, text);
	$.pop();
}
