import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let active = false;
	let primary = true;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.set_class($$element, 0, "", null, {}, {
			active,
			primary
		});
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
