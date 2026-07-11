import * as $ from "svelte/internal/client";
import Container from "./Container.svelte";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let value = $.prop($$props, "value", 15);
	Container($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const idx = $.derived(() => $$slotProps.idx);
			Child($$anchor, {
				get value() {
					return value()[$.get(idx)].name;
				},
				set value($$value) {
					value(value()[$.get(idx)].name = $$value, true);
				}
			});
		} }
	});
	$.pop();
}
