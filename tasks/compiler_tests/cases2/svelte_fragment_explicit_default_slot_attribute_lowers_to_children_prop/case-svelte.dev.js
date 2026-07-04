App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Outer($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var fragment_1 = $.comment();
			var node = $.first_child(fragment_1);
			$.add_svelte_meta(() => $.snippet(node, () => $$props.children ?? $.noop), "render", App, 8, 2);
			$.append($$anchor, fragment_1);
		}),
		$$slots: { default: true }
	}), "component", App, 6, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
