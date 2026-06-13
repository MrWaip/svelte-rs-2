import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	const $list = () => $.store_get(list, "$list", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let items = $.prop($$props, "items", 8);
	const { list } = items();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, $list, $.index, ($$anchor, item, idx) => {
		Child($$anchor, {
			get value() {
				return $list()[idx];
			},
			set value($$value) {
				$list()[idx] = $$value, $.invalidate_inner_signals(() => $list()), $.invalidate_store($$stores, "$list");
			},
			$$legacy: true
		});
	});
	$.append($$anchor, fragment);
	$$cleanup();
}
