import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	const $list = () => $.store_get(list, "$list", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let items = $.prop($$props, "items", 8);
	const { list } = items();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, $list, $.index, ($$anchor, item, idx) => {
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $list()[idx]));
		$.event("click", button, () => ($list()[idx] = $list()[idx] + 1, $.invalidate_inner_signals(() => $list()), $.invalidate_store($$stores, "$list")));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
	$$cleanup();
}
