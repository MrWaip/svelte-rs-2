import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { data } = $$props;
		if (data) {
			$$renderer.push("<!--[0-->");
			const foo = data.foo;
			$$renderer.push(`<p>${$.escape(foo)}</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	});
}
