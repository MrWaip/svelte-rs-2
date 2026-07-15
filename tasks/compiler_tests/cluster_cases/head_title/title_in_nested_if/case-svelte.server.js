import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	let b = $$props["b"];
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		if (a) {
			$$renderer.push("<!--[0-->");
			if (b) {
				$$renderer.push("<!--[0-->");
				$$renderer.title(($$renderer) => {
					$$renderer.push(`<title>deep</title>`);
				});
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	});
	$.bind_props($$props, {
		a,
		b
	});
}
