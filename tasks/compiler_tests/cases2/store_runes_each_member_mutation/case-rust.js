import * as $ from "svelte/internal/client";
import { items } from "./stores";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const $items = () => $.store_get(items, "$items", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, $items, $.index, ($$anchor, item, $$index) => {
		var button = root_1();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(item).value));
		$.delegated("click", button, () => ($.get(item).value++, $.invalidate_store($$stores, "$items")));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
	$$cleanup();
}
$.delegate(["click"]);
