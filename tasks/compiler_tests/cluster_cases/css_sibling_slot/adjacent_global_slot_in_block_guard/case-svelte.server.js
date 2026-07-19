import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<div>`);
	if (true) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "default", {}, null);
		$$renderer.push(`<!--]-->`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <p class="foo">foo</p></div>`);
}
