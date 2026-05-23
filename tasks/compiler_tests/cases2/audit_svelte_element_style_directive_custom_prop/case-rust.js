import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let value = "red";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			style: "",
			[$.STYLE]: { "--accent": value }
		}));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
