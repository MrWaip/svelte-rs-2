import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const visible = $.mutable_source(void 0, true);
	let n = $.prop($$props, "n", 9);
	let cache = $.mutable_source({}, true);
	function bump(i) {
		cache[i] = i;
	}
	$.legacy_pre_effect(() => (cache, $.deep_read_state(n())), () => {
		$.set(visible, compute(cache, n()));
	});
	$.legacy_pre_effect_reset();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(visible)));
	$.event("click", button, () => bump(1));
	$.append($$anchor, button);
	$.pop();
}
