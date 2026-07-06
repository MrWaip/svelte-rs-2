import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Form {
			name = "";
			email = "";
			data = {};
		}
		let f = new Form();
		$$renderer.push(`<p>${$.escape(f.name)}</p>`);
	});
}
