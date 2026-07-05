App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
var root = $.add_locations($.from_html(`<p>child content</p>`), App[$.FILENAME], [[10, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let ref = $.tag($.state(void 0), "ref");
	let plainRef;
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.bind_this(Component(node, {}), ($$value) => $.set(ref, $$value, true), () => $.get(ref)), "component", App, 7, 0, { componentTag: "Component" });
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => $.bind_this(Component(node_1, {}), ($$value) => plainRef = $$value, () => plainRef), "component", App, 8, 0, { componentTag: "Component" });
	var node_2 = $.sibling(node_1, 2);
	$.add_svelte_meta(() => $.bind_this(Component(node_2, {
		name: "test",
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var p = root();
			$.append($$anchor, p);
		}),
		$$slots: { default: true }
	}), ($$value) => $.set(ref, $$value, true), () => $.get(ref)), "component", App, 9, 0, { componentTag: "Component" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
