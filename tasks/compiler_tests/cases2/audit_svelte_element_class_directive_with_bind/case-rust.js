import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let active = false;
	let el = $.state(void 0);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.bind_this($$element, ($$value) => $.set(el, $$value, true), () => $.get(el));
		$.set_class($$element, 0, "", null, {}, { active });
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
