import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
import Bar from "./Bar.svelte";
var root = $.add_locations($.from_html(`<span>child</span>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => x() ? Foo : Bar, ($$anchor, $$component) => {
		$$component($$anchor, {
			answer: 42,
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				var span = root();
				$.append($$anchor, span);
			}),
			$$slots: { default: true }
		});
	}), "component", App, 6, 0, { componentTag: "svelte:component" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
