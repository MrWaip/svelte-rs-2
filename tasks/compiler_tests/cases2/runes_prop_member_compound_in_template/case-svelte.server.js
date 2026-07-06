import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { obj = { x: 0 } } = $$props;
		obj.x;
		$$renderer.push(`<!---->${$.escape(obj.x += 5)}`);
	});
}
