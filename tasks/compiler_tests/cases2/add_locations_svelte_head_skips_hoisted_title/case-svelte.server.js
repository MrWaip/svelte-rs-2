import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { title = "x" } = $$props;
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>${$.escape(title)}</title>`);
		});
		$$renderer.push(`<meta name="x" content="y"/> <link rel="canonical" href="/"/>`);
	});
}
