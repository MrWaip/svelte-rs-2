import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<select>`);
	$$renderer.option({ value: true }, ($$renderer) => {
		$$renderer.push(`a`);
	});
	$$renderer.option({}, ($$renderer) => {
		$$renderer.push(`b`);
	});
	$$renderer.push(`</select>`);
}
