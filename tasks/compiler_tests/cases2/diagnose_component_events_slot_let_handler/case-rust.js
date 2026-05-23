import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
import Button from "./Button.svelte";
export default function App($$anchor) {
	Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const onClick = $.derived_safe_equal(() => $$slotProps.onClick);
			Button($$anchor, { $$events: { click(...$$args) {
				$.get(onClick)?.apply(this, $$args);
			} } });
		} }
	});
}
