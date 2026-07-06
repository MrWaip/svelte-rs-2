import * as $ from "svelte/internal/server";
import { foo } from "lib";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let total = 0;
		let c = 0;
		let d = 0;
		$: total = c + foo(d);
		$$renderer.push(`<input${$.attr("value", c)}/> <input${$.attr("value", d)}/>`);
	});
}
