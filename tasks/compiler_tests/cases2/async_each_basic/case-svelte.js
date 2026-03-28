import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	async function getItems() {
		return [
			1,
			2,
			3
		];
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [getItems], (node, $$collection) => {
		$.each(node, 17, () => $.get($$collection), $.index, ($$anchor, item) => {
			var p = root_1();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, p);
		});
	});
	$.append($$anchor, fragment);
}
