import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function fn() {
		return 1;
	}
	$$renderer.push(`<div>`);
	$$renderer.push(async () => $.escape((await $.save("x"))()));
	$$renderer.push(`</div><div${$.attr("id", fn())}>y</div>`);
}
