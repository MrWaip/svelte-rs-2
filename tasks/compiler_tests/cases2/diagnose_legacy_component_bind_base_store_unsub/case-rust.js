import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $value = () => $.store_get($.get(value), "$value", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const x = $.mutable_source();
	let value = $.mutable_source(writable(undefined));
	$.legacy_pre_effect(() => $value(), () => {
		$.set(x, $value());
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
