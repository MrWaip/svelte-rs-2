import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let a = $.prop($$props, "a", 8);
	const c = 1;
	$.legacy_pre_effect(() => $.deep_read_state(a()), () => {
		a();
		c;
	});
	$.legacy_pre_effect_reset();
	var $$exports = { c };
	$.bind_prop($$props, "c", c);
	return $.pop($$exports);
}
