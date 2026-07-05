import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 1;
	let color = "red";
	if (count > 0) {
		$$renderer.push("<!--[0-->");
		$.css_props($$renderer, true, { "--my-color": color }, () => {
			App($$renderer, {});
		});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
