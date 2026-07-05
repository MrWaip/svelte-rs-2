import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $store = () => ($.validate_store($.get(store), "store"), $.store_get($.get(store), "$store", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const store = $.mutable_source();
	const doubled = $.mutable_source();
	let source = $.prop($$props, "source", 8);
	$.legacy_pre_effect(() => $.deep_read_state(source()), () => {
		$.store_unsub($.set(store, source()), "$store", $$stores);
	});
	$.legacy_pre_effect(() => $store(), () => {
		$.set(doubled, $store() * 2);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		get value() {
			return $.get(store);
		},
		get other() {
			return $.get(doubled);
		}
	}), "component", App, 9, 0, { componentTag: "Child" });
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
