import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	const $error = () => $.store_get(error(), "$error", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let error = $.prop($$props, "error", 8);
	Child($$anchor, {
		get value() {
			$.mark_store_binding();
			return $error();
		},
		set value($$value) {
			$.store_set(error(), $$value);
		},
		$$legacy: true
	});
	$$cleanup();
}
