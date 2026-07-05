import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let show = true;
	const promise = fetch("/api");
	if (show) {
		$$renderer.push("<!--[0-->");
		$.await($$renderer, promise, () => {}, (value) => {
			$$renderer.push(`<p>${$.escape(value)}</p>`);
		});
		$$renderer.push(`<!--]-->`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
