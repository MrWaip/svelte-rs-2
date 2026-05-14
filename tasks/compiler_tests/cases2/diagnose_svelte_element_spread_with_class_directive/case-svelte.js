import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let props = $.proxy({ id: "bar" });
	let active = false;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			...props,
			[$.CLASS]: { active }
		}));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
