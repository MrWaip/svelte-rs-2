import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let item = $$props["item"];
		$$renderer.push(`<!---->`);
		{
			$$renderer.push(`<div>${$.escape(item.id)}</div>`);
		}
		$$renderer.push(`<!---->`);
		$.bind_props($$props, { item });
	});
}
