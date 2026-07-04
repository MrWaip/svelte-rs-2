import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Holder from "./Holder.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	async function load() {
		const { default: Inner } = (await $.track_reactivity_loss(import("./Inner.svelte")))();
		return Inner;
	}
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Holder($$anchor, {
		task: load,
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const Inner = $.derived_safe_equal(() => $$slotProps.value);
			$.add_svelte_meta(() => $.get(Inner)($$anchor, {}), "component", App, 11, 1, { componentTag: "Inner" });
		} }
	}), "component", App, 10, 0, { componentTag: "Holder" });
	return $.pop($$exports);
}
