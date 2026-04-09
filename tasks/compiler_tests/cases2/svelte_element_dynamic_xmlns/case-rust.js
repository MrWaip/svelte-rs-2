import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "rect";
	let ns = "http://www.w3.org/2000/svg";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			xmlns: ns,
			width: "100",
			height: "100"
		}));
	}, () => ns);
	$.append($$anchor, fragment);
}
