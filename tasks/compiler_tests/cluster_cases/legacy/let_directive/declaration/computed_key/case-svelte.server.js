import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	const k = "z";
	Inner($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { item: { [k]: v } }) => {
			$$renderer.push(`<!---->${$.escape(v)}`);
		} }
	});
}
