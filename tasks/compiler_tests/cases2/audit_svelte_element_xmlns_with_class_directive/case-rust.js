import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "rect";
	let active = false;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, true, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			xmlns: "http://www.w3.org/2000/svg",
			class: "",
			[$.CLASS]: { active }
		}));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
