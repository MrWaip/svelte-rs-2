import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $store = () => $.store_get(store(), "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = $.prop($$props, "store", 8);
	$.init();
	Child($$anchor, {
		get value() {
			return $store().field;
		},
		set value($$value) {
			$.store_mutate(store(), $.untrack($store).field = $$value, $.untrack($store));
		},
		$$legacy: true
	});
	$.pop();
	$$cleanup();
}
