import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const y = $.mutable_source();
	let x = 1;
	$.get(y);
	$.legacy_pre_effect(() => {}, () => {
		$.set(y, x * 2);
	});
	$.legacy_pre_effect_reset();
	$.pop();
}
