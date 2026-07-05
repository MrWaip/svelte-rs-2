import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { item } = $$props;
	if (item) {
		$$renderer.push("<!--[0-->");
		const { name, label = "fallback" } = item;
		$$renderer.push(`<p>${$.escape(name)} ${$.escape(label)}</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
