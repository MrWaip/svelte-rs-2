import * as $ from "svelte/internal/server";
import List from "./List.svelte";
export default function App($$renderer) {
	List($$renderer, { $$slots: { row: ($$renderer, { row }) => {
		{
			const v = row.value * 2;
			$$renderer.push(`<p>${$.escape(v)}</p>`);
		}
	} } });
}
