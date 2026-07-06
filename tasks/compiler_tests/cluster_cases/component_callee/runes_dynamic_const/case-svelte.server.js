import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { component } = $$props;
	if (component) {
		$$renderer.push("<!--[0-->");
		const Component = component;
		if (Component) {
			$$renderer.push("<!--[-->");
			Component($$renderer, {});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
