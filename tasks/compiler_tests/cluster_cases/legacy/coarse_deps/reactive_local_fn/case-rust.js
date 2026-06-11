import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let a = $.prop($$props, "a", 8);
	function foo() {}
	$.legacy_pre_effect(() => $.deep_read_state(a()), () => {
		a();
		foo();
	});
	$.legacy_pre_effect_reset();
	$.pop();
}
