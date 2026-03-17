import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "button";
	let count = $.state(0);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		var event_handler = () => $.update(count);
		$.attribute_effect($$element, () => ({ onclick: event_handler }));
		var text = $.text("Click");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
