import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let props = $.proxy({ id: "x" });
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({ ...props }), void 0, void 0, void 0, "svelte-16b9921");
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
