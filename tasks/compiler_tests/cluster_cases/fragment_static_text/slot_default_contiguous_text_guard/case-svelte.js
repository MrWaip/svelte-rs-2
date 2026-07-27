import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<x slot="s">1</x>`);
export default function App($$anchor) {
	C($$anchor, {
		children: ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text("OMG");
			$.append($$anchor, text);
		},
		$$slots: {
			default: true,
			s: ($$anchor, $$slotProps) => {
				var x = root();
				$.append($$anchor, x);
			}
		}
	});
}
