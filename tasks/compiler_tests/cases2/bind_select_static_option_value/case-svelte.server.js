import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let selected = "a";
	$$renderer.select({ value: selected }, ($$renderer) => {
		$$renderer.option({ value: "a" }, ($$renderer) => {
			$$renderer.push(`A`);
		});
		$$renderer.option({ value: "b" }, ($$renderer) => {
			$$renderer.push(`B`);
		});
	});
}
