import * as $ from "svelte/internal/client";
import { items } from "./stores";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	const $items = () => $.store_get(items, "$items", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, $items, $.index, ($$anchor, item) => {
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
	$$cleanup();
}
