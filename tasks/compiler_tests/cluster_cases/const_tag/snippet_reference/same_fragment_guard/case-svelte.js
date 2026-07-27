import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	Comp($$anchor, {
		children: ($$anchor, $$slotProps) => {
			const foo = $.derived(() => "bar");
			$.next();
			var text = $.text();
			text.nodeValue = $.get(foo);
			$.append($$anchor, text);
		},
		$$slots: { default: true }
	});
}
