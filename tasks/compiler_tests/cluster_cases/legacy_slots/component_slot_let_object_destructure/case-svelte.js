import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Parent from "./Parent.svelte";
import Child from "./Child.svelte";
export default function App($$anchor) {
	Parent($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const foo = $.derived(() => {
			let { a, b } = $$slotProps.foo;
			return {
				a,
				b
			};
		});
		Child($$anchor, {
			slot: "item",
			children: $.invalid_default_snippet,
			$$slots: { default: ($$anchor, $$slotProps) => {
				$.next();
				var text = $.text();
				$.template_effect(() => $.set_text(text, `${$.get(foo).a ?? ""}${$.get(foo).b ?? ""}`));
				$.append($$anchor, text);
			} }
		});
	} } });
}
