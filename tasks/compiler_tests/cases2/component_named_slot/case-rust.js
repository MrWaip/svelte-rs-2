import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root_1 = $.from_html(`<p slot="footer">Footer</p>`);
export default function App($$anchor) {
	Widget($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var p = root_1();
			$.append($$anchor, p);
		},
		$$slots: { default: true }
	});
}
