import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $.fallback($$props["a"], 1);
	let b = $.fallback($$props["b"], 2);
	$$renderer.push(`<!---->${$.escape(a)} ${$.escape(a + b)}`);
	$.bind_props($$props, {
		a,
		b
	});
}
