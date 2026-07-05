import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	if (foo) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`a`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { foo });
}
