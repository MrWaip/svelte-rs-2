import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer) {
	let value = { name: "a" };
	if (true) {
		$$renderer.push("<!--[0-->");
		const item = value;
		Comp($$renderer, { name: item.name });
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
