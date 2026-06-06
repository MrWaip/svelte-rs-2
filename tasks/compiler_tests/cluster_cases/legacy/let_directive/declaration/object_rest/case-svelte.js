import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor) {
	Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const item = $.derived(() => {
				let { a, ...rest } = $$slotProps.item;
				return {
					a,
					rest
				};
			});
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, `${$.get(item).a ?? ""}${$.get(item).rest ?? ""}`));
			$.append($$anchor, text);
		} }
	});
}
