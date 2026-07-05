import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $list = () => ($.validate_store(list, "list"), $.store_get(list, "$list", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let items = $.prop($$props, "items", 8);
	const { list } = items();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, $list, $.index, ($$anchor, item, idx) => {
		$.add_svelte_meta(() => Child($$anchor, {
			get value() {
				return $list()[idx];
			},
			set value($$value) {
				$list()[idx] = $$value, $.invalidate_inner_signals(() => $list()), $.invalidate_store($$stores, "$list");
			},
			$$legacy: true
		}), "component", App, 8, 1, { componentTag: "Child" });
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
