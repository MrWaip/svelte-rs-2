import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const maxLength = $.mutable_source();
	let step = $.prop($$props, "step", 8);
	function noop() {}
	$.legacy_pre_effect(() => $.deep_read_state(step()), () => {
		$.set(maxLength, step().maxLength ?? Infinity);
	});
	$.legacy_pre_effect_reset();
	$.init();
	$.pop();
}
