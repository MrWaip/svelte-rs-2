import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = null;
	if (obj) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<p>${$.escape(obj.name)}</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
