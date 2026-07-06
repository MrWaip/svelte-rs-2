import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { props } = $$props;
	if (true) {
		$$renderer.push("<!--[0-->");
		const { x } = props;
		$$renderer.push(`<p>${$.escape(x)}</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
