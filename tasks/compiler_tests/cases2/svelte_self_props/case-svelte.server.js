import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 1;
	if (count > 0) {
		$$renderer.push("<!--[0-->");
		App($$renderer, { count: count - 1 });
		$$renderer.push(`<!---->`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
