import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.select({ value: "dog" }, ($$renderer) => {
		$$renderer.option({ value: "" }, ($$renderer) => {
			$$renderer.push(`--Please choose--`);
		});
		$$renderer.option({ value: "dog" }, ($$renderer) => {
			$$renderer.push(`Dog`);
		});
		$$renderer.option({ value: "cat" }, ($$renderer) => {
			$$renderer.push(`Cat`);
		});
	});
}
