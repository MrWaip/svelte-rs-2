import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let dynamicEl;
	let counter = 0;
	$$renderer.push(`<div${$.attr_class("", void 0, { "state": counter > 0 })}>`);
	if (counter) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<span>x</span>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--></div>`);
}
