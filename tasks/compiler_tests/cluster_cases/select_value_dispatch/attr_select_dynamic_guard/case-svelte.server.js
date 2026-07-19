import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let v = "dog";
	$$renderer.select({ value: v }, ($$renderer) => {
		$$renderer.option({ value: "dog" }, ($$renderer) => {
			$$renderer.push(`Dog`);
		});
		$$renderer.option({ value: "cat" }, ($$renderer) => {
			$$renderer.push(`Cat`);
		});
	});
	$$renderer.push(` <button>swap</button>`);
}
