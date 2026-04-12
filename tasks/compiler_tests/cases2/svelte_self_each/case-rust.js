import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let items = [1, 2];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		App(node_1, { get value() {
			return $.get(item);
		} });
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
