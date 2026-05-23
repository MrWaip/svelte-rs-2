import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let s = "color: red";
	let fs = "12px";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			style: s,
			[$.STYLE]: { "font-size": fs }
		}));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
