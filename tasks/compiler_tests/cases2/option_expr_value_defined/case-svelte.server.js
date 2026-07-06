import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 1;
	function inc() {
		count++;
	}
	$$renderer.push(`<button>+</button> <select>`);
	$$renderer.option({ value: `x-${count}` }, ($$renderer) => {
		$$renderer.push(`A`);
	});
	$$renderer.push(`</select>`);
}
