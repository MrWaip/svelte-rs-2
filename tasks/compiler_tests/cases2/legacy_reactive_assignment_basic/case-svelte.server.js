import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let sum;
		let label = $.fallback($$props["label"], "sum");
		let a = 1;
		let b = 2;
		$: sum = a + b;
		$: console.log(`${label}: ${sum}`);
		$: ((param) => {
			via_iife = param * 2;
		})(sum);
		$$renderer.push(`<p>${$.escape(sum)}-${$.escape(via_iife)}</p>`);
		$.bind_props($$props, { label });
	});
}
