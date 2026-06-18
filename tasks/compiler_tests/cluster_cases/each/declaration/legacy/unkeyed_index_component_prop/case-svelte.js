import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor, $$props) {
	let widgets = $.prop($$props, "widgets", 24, () => [{ name: "foo" }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, widgets, $.index, ($$anchor, widget, i) => {
		Widget($$anchor, {
			get widget() {
				return $.get(widget);
			},
			index: i
		});
	});
	$.append($$anchor, fragment);
}
