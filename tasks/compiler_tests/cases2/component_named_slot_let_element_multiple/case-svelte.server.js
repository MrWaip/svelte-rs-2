import * as $ from "svelte/internal/server";
import List from "./List.svelte";
export default function App($$renderer) {
	List($$renderer, { $$slots: { item: ($$renderer, { item: { text }, index }) => {
		$$renderer.push(`<p slot="item">${$.escape(text)} ${$.escape(index)}</p>`);
	} } });
}
