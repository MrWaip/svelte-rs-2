import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let a = $.prop($$props, "a", 8);
	let s = $.mutable_source(0);
	function inc() {
		$.update(s);
	}
	$.legacy_pre_effect(() => ($.deep_read_state(a()), $.get(s)), () => {
		a();
		$.get(s);
	});
	$.legacy_pre_effect_reset();
	$.pop();
}
