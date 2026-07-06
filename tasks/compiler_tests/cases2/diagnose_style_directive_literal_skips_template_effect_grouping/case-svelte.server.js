import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let item = $$props["item"];
		$$renderer.push(`<div${$.attr_style("", { display: "flex" })}>${$.escape(item.text)}</div>`);
		$.bind_props($$props, { item });
	});
}
