import * as $ from "svelte/internal/server";
import Parent from "./Parent.svelte";
export default function App($$renderer) {
	Parent($$renderer, { $$slots: { item: ($$renderer, { item }) => {
		$$renderer.push(`<div slot="item">${$.escape(item)}</div>`);
	} } });
}
