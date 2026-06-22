import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let a = $.prop($$props, "a", 8);
	let b = $.prop($$props, "b", 8);
	$.legacy_pre_effect(() => ($.deep_read_state(a()), $.deep_read_state(b())), () => {
		a();
		b();
	});
	$.legacy_pre_effect_reset();
	$.pop();
}
