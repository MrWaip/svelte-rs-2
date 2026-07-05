App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $search = () => ($.validate_store($.get(search), "search"), $.store_get($.get(search), "$search", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	function makeStore() {
		return { search: null };
	}
	let outer = void 0;
	const $$d = $.derived(() => makeStore(outer)), search = $.tag($.derived(() => $.get($$d).search), "search");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		get value() {
			$.mark_store_binding();
			return $search();
		},
		set value($$value) {
			$.store_set($.get(search), $$value);
		}
	}), "component", App, 8, 0, { componentTag: "Child" });
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
