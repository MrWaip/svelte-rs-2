import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let props = $.proxy({ id: "x" });
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({ ...props }));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
