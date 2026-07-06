import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let component = $.fallback($$props["component"], undefined);
	if (component) {
		$$renderer.push("<!--[0-->");
		const Component = component;
		Component($$renderer, {});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { component });
}
