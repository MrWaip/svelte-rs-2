import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { test } = $$props;
		$$renderer.push(`<button></button>`);
	});
}
