import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<a slot="s">1</a>`);
export default function App($$anchor) {
	C($$anchor, {
		children: ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text();
			text.nodeValue = `${x ?? ""}${y ?? ""}`;
			$.append($$anchor, text);
		},
		$$slots: {
			default: true,
			s: ($$anchor, $$slotProps) => {
				var a = root();
				$.append($$anchor, a);
			}
		}
	});
}
