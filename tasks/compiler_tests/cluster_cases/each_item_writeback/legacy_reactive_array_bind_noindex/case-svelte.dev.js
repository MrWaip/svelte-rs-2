import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let list = $.tag($.mutable_source([{}]), "list");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(list), $.index, ($$anchor, item, $$index) => {
		$.add_svelte_meta(() => Child($$anchor, {
			get value() {
				return $.get(list)[$$index];
			},
			set value($$value) {
				$.get(list)[$$index] = $$value, $.invalidate_inner_signals(() => $.get(list));
			},
			$$legacy: true
		}), "component", App, 7, 1, { componentTag: "Child" });
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
