import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let trigger = $.prop($$props, "trigger", 8);
	let value = $.mutable_source();
	function read() {
		return $.get(value);
	}
	$.legacy_pre_effect(() => $.deep_read_state(trigger()), () => {
		$.set(value, trigger());
	});
	$.legacy_pre_effect_reset();
	var button = root();
	$.event("click", button, read);
	$.append($$anchor, button);
	$.pop();
}
