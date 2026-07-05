import * as $ from "svelte/internal/server";
import { handler } from "./handlers.js";
export default function App($$renderer) {
	$$renderer.push(`<!--[-->`);
	{
		$$renderer.push(`<p>content</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
