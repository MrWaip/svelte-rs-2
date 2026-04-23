import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
var root_2 = $.from_html(`<div slot="footer"></div>`);
export default function App($$anchor) {
	let counter = 0;
	Inner($$anchor, {
		children: ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text("default text");
			$.append($$anchor, text);
		},
		$$slots: {
			default: true,
			footer: ($$anchor, $$slotProps) => {
				var div = root_2();
				div.textContent = "Footer: 0";
				$.append($$anchor, div);
			}
		}
	});
}
