import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => [
		1,
		2,
		3
	], $.index, ($$anchor, item) => {
		var root = $.template(` <div></div>`);
		var fragment_1 = root();
		var text = $.first_child(fragment_1);
		text.nodeValue = `${item ?? ""} `;
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
