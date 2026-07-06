import * as $ from "svelte/internal/server";
import A from "./A.svelte";
export default function App($$renderer) {
	let current = A;
	if (current) {
		$$renderer.push("<!--[-->");
		current($$renderer, { answer: 42 });
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
