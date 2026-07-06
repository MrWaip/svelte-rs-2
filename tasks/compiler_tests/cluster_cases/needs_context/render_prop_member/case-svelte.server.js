import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { snippets } = $$props;
		snippets.foo($$renderer);
		$$renderer.push(`<!---->`);
	});
}
