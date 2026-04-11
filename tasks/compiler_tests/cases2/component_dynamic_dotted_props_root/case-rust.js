import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => $$props.registry.Widget, ($$anchor, registry_Widget) => {
		registry_Widget($$anchor, {});
	});
	$.append($$anchor, fragment);
}
