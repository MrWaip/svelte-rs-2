import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<select>`);
	$$renderer.option({}, ($$renderer) => {
		$$renderer.push(`<div>text</div>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`</select>`);
}
