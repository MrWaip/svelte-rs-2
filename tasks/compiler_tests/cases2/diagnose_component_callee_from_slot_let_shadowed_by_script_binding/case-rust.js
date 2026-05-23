import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Holder from "./Holder.svelte";
export default function App($$anchor) {
	async function load() {
		const { default: Inner } = await import("./Inner.svelte");
		return Inner;
	}
	Holder($$anchor, {
		task: load,
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const Inner = $.derived_safe_equal(() => $$slotProps.value);
			$.get(Inner)($$anchor, {});
		} }
	});
}
