import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => $$props.Widget, ($$anchor, Widget_1) => {
		Widget_1($$anchor, {});
	});
	$.append($$anchor, fragment);
}
