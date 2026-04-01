import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor) {
	const registry = { Widget };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => registry.Widget, ($$anchor, registry_Widget) => {
		registry_Widget($$anchor, {});
	});
	$.append($$anchor, fragment);
}
