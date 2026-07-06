import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<svg><text>`);
	if (cond) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`hello`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> `);
	if (cond) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`world`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--></text></svg>`);
}
