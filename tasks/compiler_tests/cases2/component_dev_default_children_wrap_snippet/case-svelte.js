import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
export default function App($$anchor) {
	Button($$anchor, {
		children: ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text("hello");
			$.append($$anchor, text);
		},
		$$slots: { default: true }
	});
}
