App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => $$props.Widget, ($$anchor, Widget_1) => {
		Widget_1($$anchor, {});
	}), "component", App, 7, 0, { componentTag: "Widget" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
