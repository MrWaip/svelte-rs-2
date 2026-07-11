import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let condition = $$props["condition"];
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		if (condition) {
			$$renderer.push("<!--[0-->");
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>woo</title>`);
			});
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<meta id="m" name="title" content="woo"/>`);
		}
		$$renderer.push(`<!--]-->`);
	});
	$.bind_props($$props, { condition });
}
