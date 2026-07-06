import * as $ from "svelte/internal/server";
import { beforeUpdate as before, afterUpdate as after } from "svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		before(() => {
			console.log("before");
		});
		after(() => {
			console.log("after");
		});
		$$renderer.push(`<p>hooks</p>`);
	});
}
