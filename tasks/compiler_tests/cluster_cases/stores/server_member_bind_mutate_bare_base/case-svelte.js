import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	const $data = () => $.store_get($.get(data), "$data", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const data = $.derived(() => $$props.store.data);
	Child($$anchor, {
		get value() {
			return $data().amount;
		},
		set value($$value) {
			$.store_mutate($.get(data), $.untrack($data).amount = $$value, $.untrack($data));
		}
	});
	$$cleanup();
}
