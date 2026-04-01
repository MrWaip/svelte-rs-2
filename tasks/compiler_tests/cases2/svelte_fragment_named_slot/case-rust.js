import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root_1 = $.from_html(`<svelte:fragment slot="footer"><p>First</p> <p>Second</p></svelte:fragment>`);
export default function App($$anchor) {
	Widget($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var svelte:fragment = root_1();
			$.next(2);
			$.reset(svelte:fragment);
			$.append($$anchor, svelte:fragment);
		},
		$$slots: { default: true }
	});
}
