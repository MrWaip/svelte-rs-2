import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let entries = $$props["entries"];
		function put(item, value) {
			entries[item.id] = value;
		}
		$$renderer.push(`<button>x</button>`);
		$.bind_props($$props, { entries });
	});
}
