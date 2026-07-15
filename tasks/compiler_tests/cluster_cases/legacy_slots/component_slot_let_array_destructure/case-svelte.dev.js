import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Parent from "./Parent.svelte";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Parent($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const foo = $.derived(() => {
			let [a, b] = $$slotProps.foo;
			return {
				a,
				b
			};
		});
		$.add_svelte_meta(() => Child($$anchor, {
			slot: "item",
			children: $.invalid_default_snippet,
			$$slots: { default: ($$anchor, $$slotProps) => {
				$.next();
				var text = $.text();
				$.template_effect(() => $.set_text(text, `${$.get(foo).a ?? ""}${$.get(foo).b ?? ""}`));
				$.append($$anchor, text);
			} }
		}), "component", App, 7, 1, { componentTag: "Child" });
	} } }), "component", App, 6, 0, { componentTag: "Parent" });
	return $.pop($$exports);
}
