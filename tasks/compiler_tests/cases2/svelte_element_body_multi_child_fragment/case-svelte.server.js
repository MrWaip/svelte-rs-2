import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { header, headerTag = "div" } = $$props;
	let cond = true;
	$.element($$renderer, headerTag, void 0, () => {
		header($$renderer);
		$$renderer.push(`<!----> `);
		if (cond) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span>x</span>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	});
}
