import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor) {
	Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const item = $.derived(() => {
				let { p: { a } = {} } = $$slotProps.item;
				return {};
			});
			$.next();
			var text = $.text();
			text.nodeValue = a;
			$.append($$anchor, text);
		} }
	});
}
