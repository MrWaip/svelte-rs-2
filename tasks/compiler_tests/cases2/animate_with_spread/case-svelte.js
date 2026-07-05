import * as $ from "svelte/internal/client";
import { flip } from "svelte/animate";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"items"
]);
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []), rest = $.rest_props($$props, rest_excludes);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 27, items, (item) => item.id, ($$anchor, item, idx) => {
		var p = root();
		$.attribute_effect(p, () => ({
			...rest,
			"data-index": `item-${$.get(idx) ?? ""}`
		}));
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(item).name));
		$.animation(p, () => flip, null);
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
