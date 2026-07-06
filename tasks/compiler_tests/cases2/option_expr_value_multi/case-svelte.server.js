import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "a";
	function pick() {
		value = "b";
	}
	$$renderer.push(`<button>pick</button> <select>`);
	$$renderer.option({ value }, ($$renderer) => {
		$$renderer.push(`A`);
	});
	$$renderer.option({ value: `prefix-${$.stringify(value)}` }, ($$renderer) => {
		$$renderer.push(`B`);
	});
	$$renderer.push(`</select>`);
}
