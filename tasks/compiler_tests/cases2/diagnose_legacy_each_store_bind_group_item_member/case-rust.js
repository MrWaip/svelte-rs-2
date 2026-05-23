import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Row from "./Row.svelte";
export default function App($$anchor, $$props) {
	const $store = () => $.store_get(store(), "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const binding_group = [];
	let store = $.prop($$props, "store", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, $store, (item) => item.id, ($$anchor, item, $$index) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			var consequent = ($$anchor) => {
				Row($$anchor, {
					get group() {
						return $.get(item).tag;
					},
					set group($$value) {
						$.get(item).tag = $$value, $.invalidate_inner_signals(() => $store()), $.invalidate_store($$stores, "$store");
					},
					$$legacy: true
				});
			};
			$.if(node_1, ($$render) => {
				if ($.get(item), $.untrack(() => $.get(item).tag)) $$render(consequent);
			});
		}
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
	$$cleanup();
}
