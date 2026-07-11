App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => $$props.rows, $.index, ($$anchor, row, i) => {
		$.validate_binding("bind:value={row.name}", [], () => $.get(row), () => "name", 7, 8);
		$.add_svelte_meta(() => Child($$anchor, {
			get value() {
				return $.get(row).name;
			},
			set value($$value) {
				$.get(row).name = $$value;
			}
		}), "component", App, 7, 1, { componentTag: "Child" });
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
