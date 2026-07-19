import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	let items = $.proxy([
		1,
		2,
		3
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item, idx) => {
		var button = root();
		button.textContent = idx;
		$.delegated("click", button, () => items[idx]++);
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
