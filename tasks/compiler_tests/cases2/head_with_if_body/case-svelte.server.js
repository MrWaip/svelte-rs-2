import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let cond = false;
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>t</title>`);
		});
	});
	if (cond) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<span>a</span>`);
	} else {
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<span>b</span>`);
	}
	$$renderer.push(`<!--]-->`);
}
