import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor) {
	Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const item = $.derived(() => {
				let { "a-b": ab, "c d": cd } = $$slotProps.item;
				return {
					ab,
					cd
				};
			});
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, `${$.get(item).ab ?? ""}${$.get(item).cd ?? ""}`));
			$.append($$anchor, text);
		} }
	});
}
