import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Holder from "./Holder.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Holder($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const Inner = $.derived_safe_equal(() => $$slotProps.value);
			$.add_svelte_meta(() => $.get(Inner)($$anchor, {}), "component", App, 6, 1, { componentTag: "Inner" });
		} }
	}), "component", App, 5, 0, { componentTag: "Holder" });
	return $.pop($$exports);
}
