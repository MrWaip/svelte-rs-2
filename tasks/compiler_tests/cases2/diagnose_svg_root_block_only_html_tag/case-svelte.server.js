import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let cond = true;
	let raw = "<circle r={5}/>";
	$$renderer.push(`${$.html(raw)}`);
	if (cond) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<g><path d="M1"></path></g>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
