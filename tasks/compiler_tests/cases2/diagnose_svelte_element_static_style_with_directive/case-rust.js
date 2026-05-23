import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let color = "red";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			style: "font-size: 12px",
			[$.STYLE]: { color }
		}));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
