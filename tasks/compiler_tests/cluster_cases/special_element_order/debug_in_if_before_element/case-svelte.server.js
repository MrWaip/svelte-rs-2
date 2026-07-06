import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	if (count) {
		$$renderer.push("<!--[0-->");
		console.log({ count });
		debugger;
		$$renderer.push(`<button>+</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
