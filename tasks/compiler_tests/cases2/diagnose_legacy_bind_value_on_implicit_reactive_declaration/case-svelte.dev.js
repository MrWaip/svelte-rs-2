import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $store = () => ($.validate_store(store(), "store"), $.store_get(store(), "$store", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const value = $.mutable_source();
	let store = $.prop($$props, "store", 8);
	$.legacy_pre_effect(() => $store(), () => {
		$.set(value, $store());
	});
	$.legacy_pre_effect(() => ($.get(value), $store(), $.deep_read_state(store())), () => {
		if ($.strict_equals($.get(value), $store(), false)) store().set($.get(value));
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	$.add_svelte_meta(() => Child($$anchor, {
		get value() {
			return $.get(value);
		},
		set value($$value) {
			$.set(value, $$value);
		},
		$$legacy: true
	}), "component", App, 10, 0, { componentTag: "Child" });
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
