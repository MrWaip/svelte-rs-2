import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let title = "hello";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.set_class($$element, 0, "my-class");
		var text = $.text();
		text.nodeValue = "Content: hello";
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
