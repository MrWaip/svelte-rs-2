import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let props = $.proxy({ id: "x" });
	function action(node) {}
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	$.element(node_1, () => tag, false, ($$element, $$anchor) => {
		$.action($$element, ($$node) => action?.($$node));
		$.attribute_effect($$element, () => ({ ...props }));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
