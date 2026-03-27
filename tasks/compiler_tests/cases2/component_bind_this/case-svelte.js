import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
var root_1 = $.from_html(`<p>child content</p>`);
var root = $.from_html(`<!> <!> <!>`, 1);
export default function App($$anchor) {
	let ref = $.state(void 0);
	let plainRef;
	var fragment = root();
	var node = $.first_child(fragment);
	$.bind_this(Component(node, {}), ($$value) => $.set(ref, $$value, true), () => $.get(ref));
	var node_1 = $.sibling(node, 2);
	$.bind_this(Component(node_1, {}), ($$value) => plainRef = $$value, () => plainRef);
	var node_2 = $.sibling(node_1, 2);
	$.bind_this(Component(node_2, {
		name: "test",
		children: ($$anchor, $$slotProps) => {
			var p = root_1();
			$.append($$anchor, p);
		},
		$$slots: { default: true }
	}), ($$value) => $.set(ref, $$value, true), () => $.get(ref));
	$.append($$anchor, fragment);
}
