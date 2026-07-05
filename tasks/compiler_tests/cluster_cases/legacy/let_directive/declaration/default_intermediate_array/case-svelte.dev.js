import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const item = $.derived(() => {
				let [[a, b] = [8, 9], c] = $$slotProps.item;
				return { c };
			});
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}${$.get(item).c ?? ""}`));
			$.append($$anchor, text);
		} }
	}), "component", App, 5, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
