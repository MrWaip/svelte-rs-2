import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
import Button from "./Button.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const onClick = $.derived_safe_equal(() => $$slotProps.onClick);
			$.add_svelte_meta(() => Button($$anchor, { $$events: { click(...$$args) {
				$.apply(() => $.get(onClick), this, $$args, App, [7, 19]);
			} } }), "component", App, 7, 1, { componentTag: "Button" });
		} }
	}), "component", App, 6, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
