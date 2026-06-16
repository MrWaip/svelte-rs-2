import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
export default function App($$anchor, $$props) {
	let tag = $.prop($$props, "tag", 8, "h1");
	Foo($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var fragment_1 = $.comment();
			var node = $.first_child(fragment_1);
			$.element(node, tag, false, ($$element, $$anchor) => {
				var text = $.text("This is default slot");
				$.append($$anchor, text);
			});
			$.append($$anchor, fragment_1);
		},
		$$slots: { default: true }
	});
}
