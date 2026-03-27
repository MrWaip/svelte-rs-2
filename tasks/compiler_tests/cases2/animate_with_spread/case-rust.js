import * as $ from "svelte/internal/client";
import { flip } from "svelte/animate";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []), rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"items"
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 27, () => items(), (item) => item.id, ($$anchor, item, idx) => {
		var p = root_1();
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
