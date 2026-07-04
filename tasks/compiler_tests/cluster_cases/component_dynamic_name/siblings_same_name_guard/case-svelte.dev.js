App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const B = $.tag($.derived(() => A), "B");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => $.get(B), ($$anchor, B_1) => {
		B_1($$anchor, {
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				$.next();
				var text = $.text("one");
				$.append($$anchor, text);
			}),
			$$slots: { default: true }
		});
	}), "component", App, 6, 0, { componentTag: "B" });
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => $.component(node_1, () => $.get(B), ($$anchor, B_2) => {
		B_2($$anchor, {
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				$.next();
				var text_1 = $.text("two");
				$.append($$anchor, text_1);
			}),
			$$slots: { default: true }
		});
	}), "component", App, 7, 0, { componentTag: "B" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
