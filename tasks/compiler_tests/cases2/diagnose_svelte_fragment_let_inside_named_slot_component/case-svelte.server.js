import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
import Inner from "./Inner.svelte";
import Leaf from "./Leaf.svelte";
export default function App($$renderer) {
	Outer($$renderer, { $$slots: { action: ($$renderer) => {
		Inner($$renderer, {
			slot: "action",
			children: $.invalid_default_snippet,
			$$slots: { default: ($$renderer, { y }) => {
				{
					Leaf($$renderer, { value: y });
				}
			} }
		});
	} } });
}
