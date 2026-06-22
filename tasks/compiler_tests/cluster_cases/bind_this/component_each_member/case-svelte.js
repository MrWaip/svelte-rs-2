import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 24, () => [{ ref: null }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, item, $$index) => {
		$.bind_this(Comp($$anchor, { $$legacy: true }), ($$value, item) => (item.ref = $$value, $.invalidate_inner_signals(() => items())), (item) => item?.ref, () => [$.get(item)]);
	});
	$.append($$anchor, fragment);
}
