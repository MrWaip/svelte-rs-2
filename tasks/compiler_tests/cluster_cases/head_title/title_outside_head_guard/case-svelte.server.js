import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let condition = $$props["condition"];
	if (condition) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<title>nothead</title>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { condition });
}
