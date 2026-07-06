import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer, $$props) {
	let a = $.fallback($$props["a"], null);
	Inner($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { value: [a] }) => {
			const x = a ? a({ k: 1 }) : null;
			$$renderer.push(`<!---->${$.escape(x)}`);
		} }
	});
	$.bind_props($$props, { a });
}
