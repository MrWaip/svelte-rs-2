import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let myId = "foo";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			class: "bar",
			id: myId
		}));
		var text = $.text("content");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
