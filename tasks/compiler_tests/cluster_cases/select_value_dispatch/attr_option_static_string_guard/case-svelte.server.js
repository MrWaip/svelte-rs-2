import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<select>`);
	$$renderer.option({ value: "dog" }, ($$renderer) => {
		$$renderer.push(`Dog`);
	});
	$$renderer.option({ value: "cat" }, ($$renderer) => {
		$$renderer.push(`Cat`);
	});
	$$renderer.push(`</select>`);
}
