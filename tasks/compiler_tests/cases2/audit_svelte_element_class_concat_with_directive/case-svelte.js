import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let tag = "div";
	let b = $.state("two");
	let active = false;
	$.user_effect(() => {
		$.set(b, $.get(b) + "");
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, () => ({
			class: `one ${$.get(b) ?? ""}`,
			[$.CLASS]: { active }
		}));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
	$.pop();
}
