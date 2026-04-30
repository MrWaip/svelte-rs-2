import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let obj = $.prop($$props, "obj", 28, () => ({ x: 0 }));
	$.init();
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, ($.deep_read_state(obj()), $.untrack(() => obj(obj().x++, true)))));
	$.append($$anchor, text);
	$.pop();
}
