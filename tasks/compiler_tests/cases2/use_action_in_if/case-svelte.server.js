import * as $ from "svelte/internal/server";
import { tooltip } from "./actions.js";
export default function App($$renderer) {
	let show = true;
	if (show) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div>text</div>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
