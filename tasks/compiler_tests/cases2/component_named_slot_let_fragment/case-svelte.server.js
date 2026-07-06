import * as $ from "svelte/internal/server";
import List from "./List.svelte";
export default function App($$renderer) {
	List($$renderer, { $$slots: { item: ($$renderer, { item }) => {
		{
			$$renderer.push(`<p>${$.escape(item.text)}</p>`);
		}
	} } });
}
