import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let component;
	if (component) {
		$$renderer.push("<!--[-->");
		component($$renderer, { $$slots: { "empty-state": ($$renderer) => {
			{
				$$renderer.push(`empty`);
			}
		} } });
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
