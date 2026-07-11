import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Parent from "./Parent.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Parent($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const item = $.derived_safe_equal(() => $$slotProps.item);
		var text = $.text();
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, text);
	} } }), "component", App, 5, 0, { componentTag: "Parent" });
	return $.pop($$exports);
}
