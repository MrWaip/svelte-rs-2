import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function getValue() {
		return 42;
	}
	$$renderer.push(`<!--[--><!---->`);
	{
		$$renderer.push(`<p>content</p>`);
	}
	$$renderer.push(`<!----><!--]-->`);
}
