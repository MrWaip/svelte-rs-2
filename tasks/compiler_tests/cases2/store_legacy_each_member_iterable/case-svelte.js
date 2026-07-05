import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	const $items = () => $.store_get(items, "$items", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = $.prop($$props, "store", 8);
	const { items } = store();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($items(), $.untrack(() => $items().list)), $.index, ($$anchor, item) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
	$$cleanup();
}
