import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { r } = $$props;
	$$renderer.push(`<select>`);
	r($$renderer);
	$$renderer.push(`<!----><!></select> <select><optgroup label="g">`);
	r($$renderer);
	$$renderer.push(`<!----><!></optgroup></select> <select>`);
	$$renderer.option({}, ($$renderer) => {
		r($$renderer);
		$$renderer.push(`<!---->`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`</select>`);
}
