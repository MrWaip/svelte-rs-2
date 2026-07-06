import * as $ from "svelte/internal/server";
import { Kind } from "./kinds";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const { item } = $$props;
		if (item.kind === Kind.A) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span>A</span>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<span>B</span>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
