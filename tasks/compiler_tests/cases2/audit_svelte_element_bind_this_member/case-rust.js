import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let obj = $.proxy({ el: null });
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.bind_this($$element, ($$value) => obj.el = $$value, () => obj.el);
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
