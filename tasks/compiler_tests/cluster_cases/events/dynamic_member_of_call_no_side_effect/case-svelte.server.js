import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let make = (name) => ({ handler: () => console.log(name) });
		$$renderer.push(`<button>go</button>`);
	});
}
