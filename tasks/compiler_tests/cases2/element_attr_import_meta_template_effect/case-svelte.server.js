import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { x } = $$props;
		$$renderer.push(`<a${$.attr("href", import.meta.env.VITE_X)}>x</a>`);
	});
}
