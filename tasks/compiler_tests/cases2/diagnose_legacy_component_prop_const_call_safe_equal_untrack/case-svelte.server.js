import * as $ from "svelte/internal/server";
import Child from "./child.svelte";
export default function App($$renderer, $$props) {
	let x = $.fallback($$props["x"], 0);
	const tracker = { click: () => 1 };
	Child($$renderer, {
		left: tracker.click(),
		children: ($$renderer) => {
			$$renderer.push(`<!---->${$.escape(x)}`);
		},
		$$slots: { default: true }
	});
	$.bind_props($$props, { x });
}
