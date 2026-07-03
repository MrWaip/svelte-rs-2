import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>+</button>`);
export default function App($$anchor, $$props) {
	let count = $.prop($$props, "count", 12);
	var button = root();
	$.template_effect(() => {
		console.log({ count: $.untrack(() => $.snapshot(count())) });
		debugger;
	});
	$.event("click", button, () => $.update_prop(count));
	$.append($$anchor, button);
}
