import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	Inner($$anchor, { $$slots: {
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
	} });
}
