import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor) {
	const k = "z";
	Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const item = $.derived(() => {
				let { [k]: v } = $$slotProps.item;
				return { v };
			});
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, $.get(item).v));
			$.append($$anchor, text);
		} }
	});
}
