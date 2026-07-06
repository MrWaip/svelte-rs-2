import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let error = void 0;
	$$renderer.push(`<!--[-->`);
	{
		$$renderer.push(`<!---->x`);
	}
	$$renderer.push(`<!--]-->`);
	$$renderer.push(` `);
	if (error) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<p>err</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
