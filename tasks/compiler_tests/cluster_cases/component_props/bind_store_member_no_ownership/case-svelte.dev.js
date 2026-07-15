import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $store = () => ($.validate_store(store(), "store"), $.store_get(store(), "$store", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = $.prop($$props, "store", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	$.add_svelte_meta(() => Child($$anchor, {
		get value() {
			return $store().field;
		},
		set value($$value) {
			$.store_mutate(store(), $.untrack($store).field = $$value, $.untrack($store));
		},
		$$legacy: true
	}), "component", App, 6, 0, { componentTag: "Child" });
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
