import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let obj = { count: 0 };
		const get_count = () => obj.count;
		$$renderer.push(`<p>${$.escape(obj.toString())}</p>`);
	});
}
