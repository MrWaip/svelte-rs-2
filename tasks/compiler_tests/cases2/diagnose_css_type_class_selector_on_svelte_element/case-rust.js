import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "button";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.set_class($$element, 0, "x svelte-14nl1h2");
		var text = $.text("hi");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
