import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	{
		const prop = ($$anchor) => {
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, foo));
			$.append($$anchor, text);
		};
		Comp($$anchor, {
			prop,
			children: ($$anchor, $$slotProps) => {
				const foo = $.derived(() => $$props.x * 2);
			},
			$$slots: {
				prop: true,
				default: true
			}
		});
	}
}
