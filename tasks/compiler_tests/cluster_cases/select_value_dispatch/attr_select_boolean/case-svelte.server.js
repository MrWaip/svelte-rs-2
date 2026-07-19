import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.select({ value: true }, ($$renderer) => {
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`a`);
		});
		$$renderer.option({}, ($$renderer) => {
			$$renderer.push(`b`);
		});
	});
}
