import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
export default function App($$anchor) {
	let items = $.mutable_source([0]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 3, () => $.get(items), (item) => item, ($$anchor, item, idx) => {
		Component($$anchor, {
			get item() {
				return $.get(items)[$.get(idx)];
			},
			set item($$value) {
				$.get(items)[$.get(idx)] = $$value, $.invalidate_inner_signals(() => $.get(items));
			},
			$$legacy: true
		});
	});
	$.append($$anchor, fragment);
}
