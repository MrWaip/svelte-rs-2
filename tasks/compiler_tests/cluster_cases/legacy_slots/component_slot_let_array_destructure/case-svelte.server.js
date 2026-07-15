import * as $ from "svelte/internal/server";
import Parent from "./Parent.svelte";
import Child from "./Child.svelte";
export default function App($$renderer) {
	Parent($$renderer, { $$slots: { item: ($$renderer, { foo: [a, b] }) => {
		Child($$renderer, {
			slot: "item",
			children: ($$renderer) => {
				$$renderer.push(`<!---->${$.escape(a)}${$.escape(b)}`);
			},
			$$slots: { default: true }
		});
	} } });
}
