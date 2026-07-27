import * as $ from "svelte/internal/server";
import Parent from "./Parent.svelte";
export default function App($$renderer, $$props) {
	let component = $$props["component"];
	Parent($$renderer, { $$slots: { item: ($$renderer, { item, index }) => {
		if (component) {
			$$renderer.push("<!--[-->");
			component($$renderer, {
				slot: "item",
				item,
				index
			});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	} } });
	$.bind_props($$props, { component });
}
