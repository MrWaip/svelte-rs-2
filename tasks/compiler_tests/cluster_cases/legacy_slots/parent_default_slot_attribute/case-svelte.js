import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.from_html(`<span slot="default">x</span>`);
export default function App($$anchor) {
	A($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var span = root();
			$.append($$anchor, span);
		},
		$$slots: { default: true }
	});
}
