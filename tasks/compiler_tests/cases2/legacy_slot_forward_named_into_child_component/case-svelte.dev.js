App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, { $$slots: {
		icon: ($$anchor, $$slotProps) => {
			var fragment_1 = $.comment();
			var node = $.first_child(fragment_1);
			$.slot(node, $$props, "icon", {}, null);
			$.append($$anchor, fragment_1);
		},
		caption: ($$anchor, $$slotProps) => {
			var fragment_2 = $.comment();
			var node_1 = $.first_child(fragment_2);
			$.slot(node_1, $$props, "caption", {}, null);
			$.append($$anchor, fragment_2);
		}
	} }), "component", App, 5, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
