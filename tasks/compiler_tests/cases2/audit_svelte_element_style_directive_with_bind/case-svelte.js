import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let color = "red";
	let el = $.state(void 0);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.bind_this($$element, ($$value) => $.set(el, $$value, true), () => $.get(el));
		$.attribute_effect($$element, () => ({
			style: "",
			[$.STYLE]: { color }
		}));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
