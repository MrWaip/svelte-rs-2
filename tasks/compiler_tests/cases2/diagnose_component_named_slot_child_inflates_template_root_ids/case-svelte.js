import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<span slot="action"></span>`);
export default function App($$anchor) {
	let x = 0;
	Wrap($$anchor, { $$slots: {
		image: ($$anchor, $$slotProps) => {
			Inner($$anchor, { slot: "image" });
		},
		action: ($$anchor, $$slotProps) => {
			var span = root_2();
			span.textContent = "0";
			$.append($$anchor, span);
		}
	} });
}
