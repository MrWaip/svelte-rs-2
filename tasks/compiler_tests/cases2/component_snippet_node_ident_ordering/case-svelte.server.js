import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { Layout, Btn, Cap } = $$props;
	$$renderer.push(`<div>`);
	{
		function footer($$renderer) {
			if (Btn) {
				$$renderer.push("<!--[-->");
				Btn($$renderer, {});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
			$$renderer.push(` <div class="cap">`);
			if (Cap) {
				$$renderer.push("<!--[-->");
				Cap($$renderer, {});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
			$$renderer.push(`</div>`);
		}
		if (Layout) {
			$$renderer.push("<!--[-->");
			Layout($$renderer, {
				footer,
				$$slots: { footer: true }
			});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	}
	$$renderer.push(`</div>`);
}
