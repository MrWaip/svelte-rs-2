import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $store = () => $.store_get($.get(store), "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const store = $.mutable_source();
	const doubled = $.mutable_source();
	let source = $.prop($$props, "source", 8);
	$.legacy_pre_effect(() => $.deep_read_state(source()), () => {
		$.store_unsub($.set(store, source()), "$store", $$stores);
	});
	$.legacy_pre_effect(() => $store(), () => {
		$.set(doubled, $store() * 2);
	});
	$.legacy_pre_effect_reset();
	Child($$anchor, {
		get value() {
			return $.get(store);
		},
		get other() {
			return $.get(doubled);
		}
	});
	$.pop();
	$$cleanup();
}
