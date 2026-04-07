import * as $ from "svelte/internal/client";
import { flip } from "svelte/animate";
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let items = $.proxy([{
		id: 1,
		name: "a"
	}, {
		id: 2,
		name: "b"
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 25, () => items, (item) => item.id, ($$anchor, item) => {
		const label = $.derived($.get(item).name.toUpperCase);
		var div = root_1();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(label)));
		$.animation(div, () => flip, null);
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
