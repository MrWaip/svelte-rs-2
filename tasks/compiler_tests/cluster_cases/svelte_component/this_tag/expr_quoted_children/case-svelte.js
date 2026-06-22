import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
import Bar from "./Bar.svelte";
var root_1 = $.from_html(`<span>child</span>`);
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "x", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => x() ? Foo : Bar, ($$anchor, $$component) => {
		$$component($$anchor, {
			answer: 42,
			children: ($$anchor, $$slotProps) => {
				var span = root_1();
				$.append($$anchor, span);
			},
			$$slots: { default: true }
		});
	});
	$.append($$anchor, fragment);
}
