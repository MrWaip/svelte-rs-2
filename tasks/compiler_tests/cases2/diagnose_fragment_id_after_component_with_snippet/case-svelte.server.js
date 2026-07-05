import * as $ from "svelte/internal/server";
import A from "./A.svelte";
import B from "./B.svelte";
export default function App($$renderer) {
	let data = null;
	let x = null;
	{
		function inner($$renderer) {
			$$renderer.push(`<!---->`);
		}
		A($$renderer, {
			inner,
			$$slots: { inner: true }
		});
	}
	$$renderer.push(`<!----> `);
	B($$renderer, {
		children: ($$renderer) => {
			if (data) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<div>c</div>`);
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		},
		$$slots: { default: true }
	});
	$$renderer.push(`<!---->`);
}
