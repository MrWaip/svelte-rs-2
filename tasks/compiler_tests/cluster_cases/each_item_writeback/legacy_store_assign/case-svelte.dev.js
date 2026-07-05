import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 1]]);
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
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $list()[idx]));
		$.event("click", button, function click() {
			return $list()[idx] = $list()[idx] + 1, $.invalidate_inner_signals(() => $list()), $.invalidate_store($$stores, "$list");
		});
		$.append($$anchor, button);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
