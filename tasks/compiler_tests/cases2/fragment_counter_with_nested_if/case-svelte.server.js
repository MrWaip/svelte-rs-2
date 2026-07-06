import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	let visible = true;
	$$renderer.push(`<div>`);
	if (visible) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<span>0</span>`);
	} else {
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<div><input${$.attr("value", count)}/></div> `);
		if (count > 10) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<h1>Big</h1>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<h2>Small</h2>`);
		}
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]--></div> <div>`);
	if (visible) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<span>0</span>`);
	} else {
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<div><input${$.attr("value", count)}/></div> `);
		if (count > 10) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<h1>Big</h1>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<h2>Small</h2>`);
		}
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]--></div>`);
}
