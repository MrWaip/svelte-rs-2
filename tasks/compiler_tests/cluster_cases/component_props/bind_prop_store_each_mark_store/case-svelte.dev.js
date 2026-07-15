App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $rows = () => ($.validate_store(rows, "rows"), $.store_get(rows, "$rows", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const rows = writable([]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, $rows, $.index, ($$anchor, row, $$index) => {
		$.validate_binding("bind:value={row.name}", [], () => ($.mark_store_binding(), $.get(row)), () => "name", 8, 8);
		$.add_svelte_meta(() => Child($$anchor, {
			get value() {
				return $.get(row).name;
			},
			set value($$value) {
				$.get(row).name = $$value, $.invalidate_store($$stores, "$rows");
			}
		}), "component", App, 8, 1, { componentTag: "Child" });
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
