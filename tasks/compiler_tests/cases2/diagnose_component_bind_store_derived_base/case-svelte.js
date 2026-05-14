import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	const $search = () => $.store_get($.get(search), "$search", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	function makeStore() {
		return { search: null };
	}
	let outer = void 0;
	const $$d = $.derived(() => makeStore(outer)), search = $.derived(() => $.get($$d).search);
	Child($$anchor, {
		get value() {
			$.mark_store_binding();
			return $search();
		},
		set value($$value) {
			$.store_set($.get(search), $$value);
		}
	});
	$$cleanup();
}
