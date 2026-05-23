import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	const $store = () => $.store_get(store(), "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = $.prop($$props, "store", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, $store, (item) => item.id, ($$anchor, item, $$index) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			var consequent = ($$anchor) => {
				var input = root_2();
				$.remove_input_defaults(input);
				$.bind_checked(input, () => $.get(item).enabled, ($$value) => ($.get(item).enabled = $$value, $.invalidate_inner_signals(() => $store()), $.invalidate_store($$stores, "$store")));
				$.append($$anchor, input);
			};
			$.if(node_1, ($$render) => {
				if ($.get(item), $.untrack(() => $.get(item).enabled)) $$render(consequent);
			});
		}
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
	$$cleanup();
}
