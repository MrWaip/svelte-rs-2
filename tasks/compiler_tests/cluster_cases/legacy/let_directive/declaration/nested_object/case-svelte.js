import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor) {
	Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const item = $.derived(() => {
				let { p: { a }, q: { b } } = $$slotProps.item;
				return {
					a,
					b
				};
			});
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, `${$.get(item).a ?? ""}${$.get(item).b ?? ""}`));
			$.append($$anchor, text);
		} }
	});
}
