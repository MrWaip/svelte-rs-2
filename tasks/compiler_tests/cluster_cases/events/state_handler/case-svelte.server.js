import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let handler = () => {};
		$$renderer.push(`<button>x</button>`);
	});
}
