import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.from_html(`<div>x</div>`);
export default function App($$anchor) {
	A($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var div = root();
			$.append($$anchor, div);
		},
		$$slots: { default: true }
	});
}
