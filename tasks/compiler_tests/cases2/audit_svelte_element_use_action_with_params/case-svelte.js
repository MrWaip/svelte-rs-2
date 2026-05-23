import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	function action(node, opts) {}
	let opts = $.proxy({ x: 1 });
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	$.element(node_1, () => tag, false, ($$element, $$anchor) => {
		$.action($$element, ($$node, $$action_arg) => action?.($$node, $$action_arg), () => opts);
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
