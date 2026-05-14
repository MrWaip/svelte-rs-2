import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./child.svelte";
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "x", 8, 0);
	const tracker = { click: () => 1 };
	{
		let $0 = $.derived_safe_equal(() => $.untrack(() => tracker.click()));
		Child($$anchor, {
			get left() {
				return $.get($0);
			},
			children: ($$anchor, $$slotProps) => {
				$.next();
				var text = $.text();
				$.template_effect(() => $.set_text(text, x()));
				$.append($$anchor, text);
			},
			$$slots: { default: true }
		});
	}
}
