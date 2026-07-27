import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!--[!-->`);
	{
		$$renderer.push(`<!---->loading`);
	}
	$$renderer.push(`<!--]-->`);
}
