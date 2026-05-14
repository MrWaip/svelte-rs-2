import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
export default function App($$anchor, $$props) {
	Outer($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var fragment_1 = $.comment();
			var node = $.first_child(fragment_1);
			$.snippet(node, () => $$props.children ?? $.noop);
			$.append($$anchor, fragment_1);
		},
		$$slots: { default: true }
	});
}
