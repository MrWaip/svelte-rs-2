import * as $ from "svelte/internal/server";
function badge($$renderer, text) {
	$$renderer.push(`<span class="badge">${$.escape(text)}</span>`);
}
export default function App($$renderer) {
	let title = "hello";
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.push(`<meta name="description" content="test"/>`);
	});
	$$renderer.push(`<div><p>hello</p> `);
	badge($$renderer, "new");
	$$renderer.push(`<!----></div>`);
}
