import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.select({
		multiple: true,
		value: "dog"
	}, ($$renderer) => {
		$$renderer.option({ value: "dog" }, ($$renderer) => {
			$$renderer.push(`Dog`);
		});
		$$renderer.option({ value: "cat" }, ($$renderer) => {
			$$renderer.push(`Cat`);
		});
	});
}
