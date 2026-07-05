import * as $ from "svelte/internal/server";
import { foo } from "lib";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let c = 0;
		$: c += foo();
		$$renderer.push(`<input${$.attr("value", c)}/>`);
	});
}
