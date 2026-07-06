import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	Inner($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { item: { p: [a, b], q: [c, d] } }) => {
			$$renderer.push(`<!---->${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}`);
		} }
	});
}
