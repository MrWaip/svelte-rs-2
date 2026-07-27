import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	function call(callback) {
		return callback();
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.child_block(async ($$renderer) => {
		if ((await $.save(call(async () => await delay(x))))()) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>truthy</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
	});
	$$renderer.push(`<!--]--> <!--[--><!---->`);
	{
		$$renderer.push(`<p>keyed</p>`);
	}
	$$renderer.push(`<!----><!--]-->`);
}
