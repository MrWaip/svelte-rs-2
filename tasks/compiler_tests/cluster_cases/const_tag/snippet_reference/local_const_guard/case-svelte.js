import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	{
		const prop = ($$anchor) => {
			const foo = $.derived(() => "bar");
			$.next();
			var text = $.text();
			text.nodeValue = $.get(foo);
			$.append($$anchor, text);
		};
		Comp($$anchor, {
			prop,
			$$slots: { prop: true }
		});
	}
}
