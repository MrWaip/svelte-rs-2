import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	function attachment(node) {}
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	$.element(node_1, () => tag, false, ($$element, $$anchor) => {
		$.attach($$element, () => attachment);
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
