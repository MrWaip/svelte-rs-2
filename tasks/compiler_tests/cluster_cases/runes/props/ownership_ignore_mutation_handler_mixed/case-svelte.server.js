import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { a, b } = $$props;
		$$renderer.push(`<button></button>`);
	});
}
