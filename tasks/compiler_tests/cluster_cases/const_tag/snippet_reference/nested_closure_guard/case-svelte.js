import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	Comp($$anchor, {
		children: ($$anchor, $$slotProps) => {
			const foo = $.derived(() => "bar");
			{
				const prop = ($$anchor) => {
					$.next();
					var text = $.text();
					text.nodeValue = $.get(foo);
					$.append($$anchor, text);
				};
				Inner($$anchor, {
					prop,
					$$slots: { prop: true }
				});
			}
		},
		$$slots: { default: true }
	});
}
