import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { cond } = $$props;
	if (cond) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<!-- inside if --> <span>visible</span>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
