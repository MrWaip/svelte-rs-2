import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { show, point } = $$props;
	if (show) {
		$$renderer.push("<!--[0-->");
		const { x, y } = point;
		$$renderer.push(`<p>${$.escape(x)}</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
