import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 24, () => [{ ref: null }]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, items, $.index, ($$anchor, item, $$index) => {
		$.add_svelte_meta(() => $.bind_this(Comp($$anchor, { $$legacy: true }), ($$value, item) => (item.ref = $$value, $.invalidate_inner_signals(() => items())), (item) => item?.ref, () => [$.get(item)]), "component", App, 7, 1, { componentTag: "Comp" });
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
