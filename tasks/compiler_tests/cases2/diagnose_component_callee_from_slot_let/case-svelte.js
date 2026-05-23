import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Holder from "./Holder.svelte";
export default function App($$anchor) {
	Holder($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const Inner = $.derived_safe_equal(() => $$slotProps.value);
			$.get(Inner)($$anchor, {});
		} }
	});
}
