import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer, $$props) {
	let { p } = $$props;
	const store = { sel: { y: 1 } };
	if (true) {
		$$renderer.push("<!--[0-->");
		const a = store.sel;
		Comp($$renderer, { foo: a.y });
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
