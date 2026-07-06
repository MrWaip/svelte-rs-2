import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { cond = true, show = true } = $$props;
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		if (show) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<meta name="a" content="b"/>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	});
	if (cond) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<button>x</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
