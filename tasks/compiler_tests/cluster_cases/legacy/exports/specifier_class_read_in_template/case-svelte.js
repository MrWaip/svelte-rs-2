import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	class Counter {}
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, ($.deep_read_state(Counter()), $.untrack(() => Counter().name))));
	$.append($$anchor, p);
	$.pop();
}
