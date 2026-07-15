import * as $ from "svelte/internal/server";
import Nested from "./Nested.svelte";
import SlotInner from "./SlotInner.svelte";
export default function App($$renderer) {
	Nested($$renderer, { $$slots: { foo: ($$renderer, { thing: data }) => {
		SlotInner($$renderer, {
			slot: "foo",
			thing: data,
			children: ($$renderer) => {
				$$renderer.push(`<div class="inner-slot">${$.escape(data)}</div>`);
			},
			$$slots: { default: true }
		});
	} } });
}
