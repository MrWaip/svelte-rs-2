App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
import Component from "./Component.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Component($$anchor, {
		get value() {
			$.mark_store_binding();
			return $count();
		},
		set value($$value) {
			$.store_set(count, $$value);
		}
	}), "component", App, 6, 0, { componentTag: "Component" });
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
