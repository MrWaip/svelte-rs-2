import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $.fallback($$props["a"], 0);
	let b = $.fallback($$props["b"], 0);
	let flag = $.fallback($$props["flag"], false);
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "default", {
		sum: a + b,
		neg: !flag,
		both: a && b,
		cond: flag ? a : b
	}, null);
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, {
		a,
		b,
		flag
	});
}
