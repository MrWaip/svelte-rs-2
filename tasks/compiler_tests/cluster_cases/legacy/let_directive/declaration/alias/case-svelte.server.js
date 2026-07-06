import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	Inner($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { item: { a: x, b: y } }) => {
			$$renderer.push(`<!---->${$.escape(x)}${$.escape(y)}`);
		} }
	});
}
