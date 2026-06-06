import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor) {
	Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const item = $.derived(() => {
				let [a = 10, b = 20] = $$slotProps.item;
				return {};
			});
			$.next();
			var text = $.text();
			text.nodeValue = `${a ?? ""}${b ?? ""}`;
			$.append($$anchor, text);
		} }
	});
}
