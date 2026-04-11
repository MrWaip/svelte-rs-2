import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
export default function App($$anchor) {
	let count = $.state(0);
	{
		let $0 = $.derived(() => () => $.update(count));
		Button($$anchor, {
			theme: "primary",
			get onclick() {
				return $.get($0);
			},
			children: ($$anchor, $$slotProps) => {
				$.next();
				var text = $.text();
				$.template_effect(() => $.set_text(text, `Clicked ${$.get(count) ?? ""} times`));
				$.append($$anchor, text);
			},
			$$slots: { default: true }
		});
	}
}
