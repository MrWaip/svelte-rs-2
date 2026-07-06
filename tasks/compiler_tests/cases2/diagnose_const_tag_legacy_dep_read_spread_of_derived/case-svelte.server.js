import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let cond = $.fallback($$props["cond"], false);
	if (cond) {
		$$renderer.push("<!--[0-->");
		const xs = [cond ? 1 : 2];
		const ys = [3];
		const all = [...xs, ...ys];
		$$renderer.push(`${$.escape(all.length)}`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { cond });
}
