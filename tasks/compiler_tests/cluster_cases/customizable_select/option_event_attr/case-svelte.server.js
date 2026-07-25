import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "a";
	function onclick() {}
	$$renderer.select({ value }, ($$renderer) => {
		$$renderer.option({
			value: "a",
			onclick
		}, ($$renderer) => {
			$$renderer.push(`a`);
		});
	});
}
