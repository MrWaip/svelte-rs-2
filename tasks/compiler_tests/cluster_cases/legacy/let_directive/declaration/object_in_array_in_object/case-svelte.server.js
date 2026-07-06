import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	Inner($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { item: { outer: [{ inner }] } }) => {
			$$renderer.push(`<!---->${$.escape(inner)}`);
		} }
	});
}
