import * as $ from "svelte/internal/server";
import { onMount } from "svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let derived_count;
		let flag = false;
		onMount(() => {
			const cb = () => {
				flag = true;
			};
			cb();
		});
		$: derived_count = (() => {
			if (flag) {
				return 1;
			}
			return 0;
		})();
		$$renderer.push(`<p>${$.escape(derived_count)}</p>`);
	});
}
