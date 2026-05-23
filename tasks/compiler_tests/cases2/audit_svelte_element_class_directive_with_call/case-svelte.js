import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let n = 0;
	function compute(x) {
		return x > 0;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		let classes;
		$.template_effect(($0) => classes = $.set_class($$element, 0, "", null, classes, $0), [() => ({ active: compute(n) })]);
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
