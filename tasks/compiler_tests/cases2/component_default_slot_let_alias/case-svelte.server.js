import * as $ from "svelte/internal/server";
import List from "./List.svelte";
export default function App($$renderer) {
	List($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { item: processed }) => {
			$$renderer.push(`<p>${$.escape(processed.text)}</p>`);
		} }
	});
}
