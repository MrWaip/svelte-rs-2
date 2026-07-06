import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let ref;
	if (true) {
		$$renderer.push("<!--[0-->");
		App($$renderer, { answer: 42 });
		$$renderer.push(`<!---->`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
