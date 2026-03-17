import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		var text = $.text("Hello");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
