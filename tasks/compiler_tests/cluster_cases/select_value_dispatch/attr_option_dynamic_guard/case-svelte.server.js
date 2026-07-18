import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let v = "dog";
	$$renderer.push(`<select>`);
	$$renderer.option({ value: v }, ($$renderer) => {
		$$renderer.push(`Dog`);
	});
	$$renderer.push(`</select> <button>swap</button>`);
}
