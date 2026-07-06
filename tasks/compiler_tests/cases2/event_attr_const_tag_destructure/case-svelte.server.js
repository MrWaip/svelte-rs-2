import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	if (true) {
		$$renderer.push("<!--[0-->");
		const { onClick } = props;
		$$renderer.push(`<button>x</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
