import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.set_class($$element, 0, "dynamic svelte-z15oen");
		var text = $.text("content");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
