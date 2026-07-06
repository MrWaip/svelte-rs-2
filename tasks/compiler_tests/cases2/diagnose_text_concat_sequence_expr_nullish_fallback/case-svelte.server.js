import * as $ from "svelte/internal/server";
import { Kind } from "./kinds";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let item = $$props["item"];
		$$renderer.push(`<span>Prefix ${$.escape(item?.kind === Kind.A ? "one" : "two")} suffix</span>`);
		$.bind_props($$props, { item });
	});
}
