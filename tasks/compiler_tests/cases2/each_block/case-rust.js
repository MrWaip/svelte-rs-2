import * as $ from "svelte/internal/client";
var root = $.template(` <div></div>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => [
		1,
		2,
		3
	], $.index, ($$anchor, item) => {
		var fragment_1 = root();
		var text = $.first_child(fragment_1);
		text.nodeValue = `${item ?? ""} `;
		var div = $.sibling(text);
		div.textContent = `${item ?? ""} + example`;
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
