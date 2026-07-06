import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let src = "";
	let cond = false;
	function on_load() {}
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		if (cond) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<script async=""${$.attr("src", src)} onload="this.__e=event"><\/script>`);
			$$renderer.push(`<!---->`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	});
}
