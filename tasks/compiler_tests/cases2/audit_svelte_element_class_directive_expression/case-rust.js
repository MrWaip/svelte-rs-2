import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let n = 0;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.set_class($$element, 0, "", null, {}, { active: n > 0 });
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
