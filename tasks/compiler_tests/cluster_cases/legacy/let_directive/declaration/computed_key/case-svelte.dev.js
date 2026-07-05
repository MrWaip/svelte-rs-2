import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const k = "z";
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, {
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
	}), "component", App, 6, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
