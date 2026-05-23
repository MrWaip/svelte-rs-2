import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let active = false;
	function handler() {}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.set_class($$element, 0, "", null, {}, { active });
		$.event("click", $$element, handler);
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
