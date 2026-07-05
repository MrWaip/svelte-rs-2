App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const B = $.tag($.derived(() => A), "B");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => $.get(B), ($$anchor, B_1) => {
		B_1($$anchor, {
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				var fragment_1 = $.comment();
				var node_1 = $.first_child(fragment_1);
				$.add_svelte_meta(() => $.component(node_1, () => $.get(B), ($$anchor, $$component) => {
					$$component($$anchor, {
						children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
							$.next();
							var text = $.text("test");
							$.append($$anchor, text);
						}),
						$$slots: { default: true }
					});
				}), "component", App, 7, 1, { componentTag: "svelte:component" });
				$.append($$anchor, fragment_1);
			}),
			$$slots: { default: true }
		});
	}), "component", App, 6, 0, { componentTag: "B" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
