import * as $ from "svelte/internal/client";
import { count } from "./stores";
import Component from "./Component.svelte";
export default function App($$anchor) {
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	Component($$anchor, {
		get value() {
			$.mark_store_binding();
			return $count();
		},
		set value($$value) {
			$.store_set(count, $$value);
		}
	});
	$$cleanup();
}
