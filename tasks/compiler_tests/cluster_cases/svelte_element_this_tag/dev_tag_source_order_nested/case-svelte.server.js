import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let show = false;
	$.element($$renderer, "p", void 0, () => {
		$$renderer.push(`before`);
	});
	$$renderer.push(` `);
	if (show) {
		$$renderer.push("<!--[0-->");
		$.element($$renderer, "strong", void 0, () => {
			$$renderer.push(`during`);
		});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> `);
	$.element($$renderer, "p", void 0, () => {
		$$renderer.push(`after`);
	});
}
