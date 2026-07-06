import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let url = "/api";
		function outer() {
			async function inner() {
				let $$d = await $.async_derived(() => fetch(url).then((r) => r.json())), data = $.derived(() => $$d().data), meta = $.derived(() => $$d().meta);
				return 1;
			}
			return inner;
		}
		$.bind_props($$props, { outer });
	});
}
