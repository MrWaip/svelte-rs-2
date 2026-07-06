import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "a";
	$$renderer.select({ value }, ($$renderer) => {
		$$renderer.option({ value: "a" }, ($$renderer) => {
			$$renderer.push(`a`);
		});
		$$renderer.option({ value: "b" }, ($$renderer) => {
			$$renderer.push(`b`);
		});
	});
}
