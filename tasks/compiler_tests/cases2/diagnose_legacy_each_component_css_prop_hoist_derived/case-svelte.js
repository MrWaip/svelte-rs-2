import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Sticker from "./Sticker.svelte";
var root_1 = $.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 24, () => []);
	const color = (x) => x.color ?? "red";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, item) => {
		var fragment_1 = root_1();
		var node_1 = $.first_child(fragment_1);
		{
			let $0 = $.derived_safe_equal(() => ($.get(item), $.untrack(() => color($.get(item)))));
			$.css_props(node_1, () => ({ "--bg": $.get($0) }));
			Sticker(node_1.lastChild, $.spread_props(() => $.get(item)));
			$.reset(node_1);
		}
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
