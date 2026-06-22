import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
export default function App($$anchor) {
	A($$anchor, {
		children: ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text("bar");
			$.append($$anchor, text);
		},
		$$slots: { default: true }
	});
}
