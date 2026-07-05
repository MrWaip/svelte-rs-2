import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 1;
	if (count > 0) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div><p>ok</p>`);
		App($$renderer, {});
		$$renderer.push(`<!----></div>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
