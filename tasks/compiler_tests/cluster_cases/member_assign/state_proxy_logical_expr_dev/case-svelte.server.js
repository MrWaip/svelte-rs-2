import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let object = { items: null };
		$$renderer.push(`<button>items: ${$.escape(JSON.stringify(object.items))}</button>`);
	});
}
