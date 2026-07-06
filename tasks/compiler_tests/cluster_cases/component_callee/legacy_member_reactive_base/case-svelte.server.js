import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let lib = $.fallback($$props["lib"], undefined);
	if (lib) {
		$$renderer.push("<!--[0-->");
		const L = lib;
		L.Button($$renderer, {});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { lib });
}
