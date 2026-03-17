import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => "div", false, ($$element, $$anchor) => {
		var text = $.text("Hello");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
