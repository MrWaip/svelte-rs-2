import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let widgets = $.prop($$props, "widgets", 24, () => [{ name: "foo" }]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, widgets, $.index, ($$anchor, widget, i) => {
		$.add_svelte_meta(() => Widget($$anchor, {
			get widget() {
				return $.get(widget);
			},
			index: i
		}), "component", App, 7, 1, { componentTag: "Widget" });
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
