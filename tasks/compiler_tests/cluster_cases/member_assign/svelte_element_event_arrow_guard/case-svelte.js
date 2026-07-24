import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let obj = $.proxy({ x: null });
	let src = $.proxy({});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => "button", false, ($$element, $$anchor) => {
		var event_handler = () => obj.x = src;
		$.attribute_effect($$element, () => ({ onclick: event_handler }));
		var text = $.text("go");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
