import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let selected = "a";
	$$renderer.select({ value: selected }, ($$renderer) => {
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`a`);
		});
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`b`);
		});
	});
}
