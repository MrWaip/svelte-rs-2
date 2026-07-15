import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const visible = $.mutable_source(void 0, true);
	let n = $.prop($$props, "n", 9);
	let total = $.mutable_source(0, true);
	function bump() {
		$.set(total, $.get(total) + 1);
	}
	$.legacy_pre_effect(() => ($.get(total), $.deep_read_state(n())), () => {
		$.set(visible, compute($.get(total), n()));
	});
	$.legacy_pre_effect_reset();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(visible)));
	$.event("click", button, bump);
	$.append($$anchor, button);
	$.pop();
}
