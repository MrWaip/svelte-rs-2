import * as $ from "svelte/internal/server";
import data from "./dep.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let total, doubled;
		function bump() {
			data.count += 1;
		}
		$: total = data.count;
		$: doubled = total * 2;
		$$renderer.push(`<button>${$.escape(doubled)}</button>`);
	});
}
