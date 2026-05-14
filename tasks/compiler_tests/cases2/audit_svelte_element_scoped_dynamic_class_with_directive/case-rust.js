import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let cls = "a";
	let active = false;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			class: cls,
			[$.CLASS]: { active }
		}), void 0, void 0, void 0, "svelte-16bdf5m");
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
