import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $value = () => ($.validate_store($.get(value), "value"), $.store_get($.get(value), "$value", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const x = $.mutable_source();
	let value = $.tag($.mutable_source(writable(undefined)), "value");
	$.legacy_pre_effect(() => $value(), () => {
		$.set(x, $value());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	$.add_svelte_meta(() => Child($$anchor, {
		get value() {
			return $.get(value);
		},
		set value($$value) {
			$.store_unsub($.set(value, $$value), "$value", $$stores);
		},
		$$legacy: true
	}), "component", App, 10, 0, { componentTag: "Child" });
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
