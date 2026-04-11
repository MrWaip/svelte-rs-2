import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor) {
	const registry_name = { Widget };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => registry_name.Widget, ($$anchor, registry_name_Widget) => {
		registry_name_Widget($$anchor, {});
	});
	$.append($$anchor, fragment);
}
