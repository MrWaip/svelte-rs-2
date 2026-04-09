import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let items = $.prop($$props, "items", 25, () => [1, 2]);
	$.init(true);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, ($.deep_read_state(items()), $.untrack(() => items().length))));
	$.append($$anchor, p);
	$.pop();
}
