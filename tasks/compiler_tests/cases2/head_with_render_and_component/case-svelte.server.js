import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { children } = $$props;
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.push(`<style>html { height: 100%; }</style>`);
	});
	A($$renderer, {});
	$$renderer.push(`<!----> `);
	children?.($$renderer);
	$$renderer.push(`<!---->`);
}
