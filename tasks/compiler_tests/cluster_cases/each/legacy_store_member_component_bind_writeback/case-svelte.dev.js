import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $data = () => ($.validate_store(data, "data"), $.store_get(data, "$data", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const data = writable({ docs: [] });
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => ($data(), $.untrack(() => $data().docs)), $.index, ($$anchor, doc, idx) => {
		$.add_svelte_meta(() => Child($$anchor, {
			get document() {
				return ($data(), $.untrack(() => $data().docs))[idx];
			},
			set document($$value) {
				($data(), $.untrack(() => $data().docs))[idx] = $$value, $.invalidate_inner_signals(() => $data()), $.invalidate_store($$stores, "$data");
			},
			$$legacy: true
		}), "component", App, 9, 1, { componentTag: "Child" });
	}), "each", App, 8, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
