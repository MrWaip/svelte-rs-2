import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rows = $.prop($$props, "rows", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 1));
		let first = () => $.get($$array)[0];
		first();
		$.add_svelte_meta(() => Child($$anchor, {
			get value() {
				return first();
			},
			set value($$value) {
				$$array[0] = $$value, $.invalidate_inner_signals(() => rows());
			},
			$$legacy: true
		}), "component", App, 8, 1, { componentTag: "Child" });
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
