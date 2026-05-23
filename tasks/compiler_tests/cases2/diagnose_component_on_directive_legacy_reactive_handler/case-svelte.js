import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const handler = $.mutable_source();
	let cond = $.prop($$props, "cond", 8, false);
	function a() {}
	function b() {}
	$.legacy_pre_effect(() => $.deep_read_state(cond()), () => {
		$.set(handler, cond() ? a : b);
	});
	$.legacy_pre_effect_reset();
	Child($$anchor, { $$events: { click(...$$args) {
		$.get(handler)?.apply(this, $$args);
	} } });
	$.pop();
}
