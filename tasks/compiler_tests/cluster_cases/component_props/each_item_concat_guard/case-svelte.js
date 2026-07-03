import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let items = $.proxy([{ text: "t" }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, badge) => {
		Badge($$anchor, { get dataTestid() {
			return `badge-${$.get(badge).text ?? ""}`;
		} });
	});
	$.append($$anchor, fragment);
}
