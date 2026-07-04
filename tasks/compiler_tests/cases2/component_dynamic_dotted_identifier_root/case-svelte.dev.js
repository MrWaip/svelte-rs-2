App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const registry_name = { Widget };
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => registry_name.Widget, ($$anchor, registry_name_Widget) => {
		registry_name_Widget($$anchor, {});
	}), "component", App, 9, 0, { componentTag: "registry_name.Widget" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
