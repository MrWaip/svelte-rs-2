import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const props = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => fetch(1, 2, 3, $$props.field1), ($$anchor) => {});
	$.append($$anchor, fragment);
	$.pop();
}
