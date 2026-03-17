import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "p";
	let name = "world";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		var text = $.text();
		text.nodeValue = "Hello world!";
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
