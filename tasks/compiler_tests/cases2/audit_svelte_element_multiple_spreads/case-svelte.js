import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let a = $.proxy({ id: "a" });
	let b = $.proxy({ id: "b" });
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			...a,
			...b
		}));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
