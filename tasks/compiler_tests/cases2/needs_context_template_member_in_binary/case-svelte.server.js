import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { foo } = $$props;
		if (foo.bar === "x") {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`a`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`b`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
