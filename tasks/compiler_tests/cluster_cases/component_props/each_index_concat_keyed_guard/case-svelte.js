import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let items = $.proxy([{ id: 1 }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 19, () => items, (badge) => badge.id, ($$anchor, badge, i) => {
		Badge($$anchor, { get dataTestid() {
			return `badge-${$.get(i) ?? ""}`;
		} });
	});
	$.append($$anchor, fragment);
}
