import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function handleScroll() {
		count++;
	}
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>Count: ${$.escape(count)}</title>`);
		});
		$$renderer.push(`<meta name="description" content="test"/>`);
	});
	$$renderer.push(`<div>Count: ${$.escape(count)}</div>`);
}
