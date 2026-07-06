import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<p>v ${$.escape(7.36.toString())}</p>`);
	});
}
