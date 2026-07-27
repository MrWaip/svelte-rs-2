import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = $.fallback($$props["x"], 42);
		let a;
		let b;
		$: b = (function(a) {
			return a;
		})(x);
		$: a = b;
		$$renderer.push(`<p>${$.escape(a)} ${$.escape(b)}</p>`);
		$.bind_props($$props, { x });
	});
}
