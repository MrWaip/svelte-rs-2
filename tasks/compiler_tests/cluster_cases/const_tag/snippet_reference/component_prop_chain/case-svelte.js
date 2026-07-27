import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	{
		const prop = ($$anchor) => {
			$.next();
			var text = $.text();
			text.nodeValue = "2";
			$.append($$anchor, text);
		};
		Comp($$anchor, {
			prop,
			children: ($$anchor, $$slotProps) => {
				const a = $.derived(() => 1);
				const foo = $.derived(() => $.get(a) + 1);
			},
			$$slots: {
				prop: true,
				default: true
			}
		});
	}
}
