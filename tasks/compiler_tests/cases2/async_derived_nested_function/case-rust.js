import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let url = "/api";
	function outer() {
		async function inner() {
			let data = (await $.save($.async_derived(() => fetch(url))))();
			return $.get(data);
		}
		return inner;
	}
	var $$exports = { outer };
	return $.pop($$exports);
}
