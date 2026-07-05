import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Row from "./Row.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $store = () => ($.validate_store(store(), "store"), $.store_get(store(), "$store", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = $.prop($$props, "store", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, $store, (item) => item.id, ($$anchor, item, $$index) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			var consequent = ($$anchor) => {
				$.add_svelte_meta(() => Row($$anchor, {
					get value() {
						return $.get(item).value;
					},
					set value($$value) {
						$.get(item).value = $$value, $.invalidate_inner_signals(() => $store()), $.invalidate_store($$stores, "$store");
					},
					$$legacy: true
				}), "component", App, 9, 2, { componentTag: "Row" });
			};
			$.add_svelte_meta(() => $.if(node_1, ($$render) => {
				if ($.get(item), $.untrack(() => $.get(item).value)) $$render(consequent);
			}), "if", App, 8, 1);
		}
		$.append($$anchor, fragment_1);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
