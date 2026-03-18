import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
var root_1 = $.from_html(`<p>child content</p>`);
export default function App($$anchor) {
	let ref = $.state(void 0);
	$.bind_this(Component($$anchor, {
		name: "test",
		children: ($$anchor, $$slotProps) => {
			var p = root_1();
			$.append($$anchor, p);
		},
		$$slots: { default: true }
	}), ($$value) => $.set(ref, $$value, true), () => $.get(ref));
}
