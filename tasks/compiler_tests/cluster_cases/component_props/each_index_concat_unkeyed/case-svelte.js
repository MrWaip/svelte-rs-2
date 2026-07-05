import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let items = $.proxy([{ text: "t" }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, badge, i) => {
		Badge($$anchor, {
			get text() {
				return $.get(badge).text;
			},
			dataTestid: `badge-${i}`
		});
	});
	$.append($$anchor, fragment);
}
