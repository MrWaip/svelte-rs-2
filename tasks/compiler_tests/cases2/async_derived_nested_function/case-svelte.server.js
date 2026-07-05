import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let url = "/api";
	function outer() {
		async function inner() {
			let data = await $.async_derived(() => fetch(url));
			return data();
		}
		return inner;
	}
	$.bind_props($$props, { outer });
}
