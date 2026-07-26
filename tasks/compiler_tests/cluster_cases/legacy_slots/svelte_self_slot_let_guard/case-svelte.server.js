import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let depth = $.fallback($$props["depth"], 0);
	if (depth > 0) {
		$$renderer.push("<!--[0-->");
		App($$renderer, {
			depth: depth - 1,
			children: $.invalid_default_snippet,
			$$slots: { default: ($$renderer, { item, index }) => {
				$$renderer.push(`<p>${$.escape(item)} ${$.escape(index)}</p>`);
			} }
		});
		$$renderer.push(`<!---->`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { depth });
}
