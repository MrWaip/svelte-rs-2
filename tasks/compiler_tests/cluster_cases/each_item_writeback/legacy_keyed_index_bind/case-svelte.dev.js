import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.tag($.mutable_source([0]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 3, () => $.get(items), (item) => item, ($$anchor, item, idx) => {
		$.add_svelte_meta(() => Component($$anchor, {
			get item() {
				return $.get(items)[$.get(idx)];
			},
			set item($$value) {
				$.get(items)[$.get(idx)] = $$value, $.invalidate_inner_signals(() => $.get(items));
			},
			$$legacy: true
		}), "component", App, 7, 1, { componentTag: "Component" });
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
