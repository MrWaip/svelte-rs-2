import * as $ from "svelte/internal/server";
import { pending } from "./handlers.js";
export default function App($$renderer) {
	if (pending) {
		$$renderer.push(`<!--[!-->`);
		pending($$renderer);
		$$renderer.push(`<!--]-->`);
	} else {
		$$renderer.push(`<!--[-->`);
		{
			$$renderer.push(`<p>content</p>`);
		}
		$$renderer.push(`<!--]-->`);
	}
}
