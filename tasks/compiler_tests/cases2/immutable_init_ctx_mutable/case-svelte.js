import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let user = $.prop($$props, "user", 24, () => ({ name: "a" }));
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, ($.deep_read_state(user()), $.untrack(() => user().name))));
	$.append($$anchor, p);
	$.pop();
}
