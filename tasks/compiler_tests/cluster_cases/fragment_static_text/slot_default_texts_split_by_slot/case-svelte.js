import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`foobar`, 1);
var root_1 = $.from_html(`<x slot="s">y</x>`);
export default function App($$anchor) {
	C($$anchor, {
		children: ($$anchor, $$slotProps) => {
			$.next();
			var fragment_1 = root();
			$.append($$anchor, fragment_1);
		},
		$$slots: {
			default: true,
			s: ($$anchor, $$slotProps) => {
				var x = root_1();
				$.append($$anchor, x);
			}
		}
	});
}
