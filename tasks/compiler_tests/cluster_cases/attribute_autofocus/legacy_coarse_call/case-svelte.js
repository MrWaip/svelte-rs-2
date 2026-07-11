import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let adapter = $.prop($$props, "adapter", 8);
	let day = $.prop($$props, "day", 8);
	let focused = $.mutable_source(null);
	function pick() {
		$.set(focused, day());
	}
	$.init();
	var button = root();
	$.autofocus(button, ($.get(focused), $.deep_read_state(adapter()), $.deep_read_state(day()), $.untrack(() => $.get(focused) !== null && adapter().isSame(day(), $.get(focused)))));
	$.event("click", button, pick);
	$.append($$anchor, button);
	$.pop();
}
