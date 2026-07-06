import * as $ from "svelte/internal/server";
import { beforeUpdate, afterUpdate } from "svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		beforeUpdate(() => {
			console.log("before");
		});
		afterUpdate(() => {
			console.log("after");
		});
		$$renderer.push(`<p>hooks</p>`);
	});
}
