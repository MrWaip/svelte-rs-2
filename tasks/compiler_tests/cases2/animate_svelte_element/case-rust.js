import * as $ from "svelte/internal/client";
import { flip } from "svelte/animate";
export default function App($$anchor) {
	let tag = "div";
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
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.element(node_1, () => tag, false, ($$element, $$anchor) => {
			$.animation($$element, () => flip, null);
			var text = $.text();
			$.template_effect(() => $.set_text(text, $.get(item).name));
			$.append($$anchor, text);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
