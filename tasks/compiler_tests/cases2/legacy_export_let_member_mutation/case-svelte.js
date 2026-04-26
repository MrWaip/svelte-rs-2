import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let obj = $.prop($$props, "obj", 28, () => ({ a: 1 }));
	obj(obj().a = 99, true);
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, ($.deep_read_state(obj()), $.untrack(() => obj().a))));
	$.append($$anchor, p);
	$.pop();
}
