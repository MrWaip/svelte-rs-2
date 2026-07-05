import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	const item = getItem();
	if (item) {
		$$renderer.push("<!--[0-->");
		const { title, status } = item;
		Child($$renderer, {
			status,
			children: ($$renderer) => {
				$$renderer.push(`<!---->${$.escape(title)}`);
			},
			$$slots: { default: true }
		});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
