import * as $ from "svelte/internal/server";
import { foo } from "lib";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = {};
		let c = "";
		$: obj.purpose = (c ? c : "") + foo(obj.type);
		$$renderer.push(`<input${$.attr("value", c)}/> <input${$.attr("value", obj.x)}/>`);
	});
}
