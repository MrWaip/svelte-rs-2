import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const foo = false;
		let bar = false;
		$$renderer.push(`<p>${$.escape(foo)} ${$.escape(bar)}</p>`);
	});
}
