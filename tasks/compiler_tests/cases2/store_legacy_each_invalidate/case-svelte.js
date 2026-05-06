import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { items } from "./stores";
var root_1 = $.from_html(`<p> </p> <button>edit</button>`, 1);
export default function App($$anchor) {
	const $items = () => $.store_get(items, "$items", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, $items, $.index, ($$anchor, item, $$index) => {
		var fragment_1 = root_1();
		var p = $.first_child(fragment_1);
		var text = $.child(p, true);
		$.reset(p);
		var button = $.sibling(p, 2);
		$.template_effect(() => $.set_text(text, ($.get(item), $.untrack(() => $.get(item).name))));
		$.delegated("click", button, () => ($.get(item).name = "x", $.invalidate_inner_signals(() => $items()), $.invalidate_store($$stores, "$items")));
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
	$$cleanup();
}
$.delegate(["click"]);
