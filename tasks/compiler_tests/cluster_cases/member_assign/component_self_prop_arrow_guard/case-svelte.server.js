import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { x: null };
	let src = {};
	let depth = 0;
	if (depth) {
		$$renderer.push("<!--[0-->");
		App($$renderer, {
			onChange: (v) => obj.x = src,
			depth: depth - 1
		});
		$$renderer.push(`<!---->`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
