import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	const $derived = () => $.store_get(derived, "$derived", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let source = { value: 1 };
	let { value } = $derived()(source);
	$$cleanup();
}
