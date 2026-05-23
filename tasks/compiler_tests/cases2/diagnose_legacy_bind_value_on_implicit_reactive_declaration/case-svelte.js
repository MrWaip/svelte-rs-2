import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $store = () => $.store_get(store(), "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const value = $.mutable_source();
	let store = $.prop($$props, "store", 8);
	$.legacy_pre_effect(() => $store(), () => {
		$.set(value, $store());
	});
	$.legacy_pre_effect(() => ($.get(value), $store(), $.deep_read_state(store())), () => {
		if ($.get(value) !== $store()) store().set($.get(value));
	});
	$.legacy_pre_effect_reset();
	$.init();
	Child($$anchor, {
		get value() {
			return $.get(value);
		},
		set value($$value) {
			$.set(value, $$value);
		},
		$$legacy: true
	});
	$.pop();
	$$cleanup();
}
