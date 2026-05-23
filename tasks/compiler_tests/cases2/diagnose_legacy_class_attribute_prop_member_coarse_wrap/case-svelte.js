import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let config = $.prop($$props, "config", 8);
	$.init();
	var div = root();
	$.template_effect(() => $.set_class(div, 1, $.clsx(($.deep_read_state(config()), $.untrack(() => config().cls)))));
	$.append($$anchor, div);
	$.pop();
}
