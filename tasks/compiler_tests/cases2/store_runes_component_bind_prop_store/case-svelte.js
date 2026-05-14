import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	const $value = () => $.store_get($$props.value, "$value", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	Child($$anchor, {
		get cents() {
			$.mark_store_binding();
			return $value();
		},
		set cents($$value) {
			$.store_set($$props.value, $$value);
		}
	});
	$$cleanup();
}
