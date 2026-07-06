import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const { x } = $$props;
	$$renderer.push(`<div>`);
	Cmp($$renderer, {
		children: ($$renderer) => {
			if (x) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<span>a</span>`);
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]--> <p>tail</p>`);
		},
		$$slots: { default: true }
	});
	$$renderer.push(`<!----></div>`);
}
