import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	{
		const prop = ($$anchor) => {
			$.next();
			var text = $.text();
			text.nodeValue = "bar";
			$.append($$anchor, text);
		};
		Comp($$anchor, {
			prop,
			children: ($$anchor, $$slotProps) => {
				const foo = $.derived(() => "bar");
			},
			$$slots: {
				prop: true,
				default: true
			}
		});
	}
}
