import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { Comp } = $$props;
	$.css_props($$renderer, true, { "--my-var": "baseline" }, () => {
		{
			function element($$renderer, { idx }) {
				$$renderer.push(`<div>${$.escape(idx)}</div>`);
			}
			if (Comp) {
				$$renderer.push("<!--[-->");
				Comp($$renderer, {
					element,
					$$slots: { element: true }
				});
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		}
	}, true);
}
