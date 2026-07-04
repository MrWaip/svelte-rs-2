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
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(list), $.index, ($$anchor, item, idx) => {
		$.add_svelte_meta(() => Child($$anchor, {
			get value() {
				return $.get(list)[idx];
			},
			set value($$value) {
				$.get(list)[idx] = $$value, $.invalidate_inner_signals(() => $.get(list));
			},
			$$legacy: true
		}), "component", App, 7, 1, { componentTag: "Child" });
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
