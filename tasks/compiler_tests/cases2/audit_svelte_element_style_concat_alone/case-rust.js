import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let tag = "div";
	let c = $.state("red");
	$.user_effect(() => {
		$.set(c, $.get(c) + "");
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({ style: `color: ${$.get(c) ?? ""}` }));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
	$.pop();
}
