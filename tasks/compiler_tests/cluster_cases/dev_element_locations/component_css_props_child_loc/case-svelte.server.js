import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { Icon } = $$props;
	$$renderer.push(`<span>`);
	$.css_props($$renderer, true, { "--color": "red" }, () => {
		if (Icon) {
			$$renderer.push("<!--[-->");
			Icon($$renderer, {});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	}, true);
	$$renderer.push(`</span>`);
}
