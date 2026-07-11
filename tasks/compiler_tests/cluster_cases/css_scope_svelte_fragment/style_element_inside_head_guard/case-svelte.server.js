import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.push(`<style>
    body { background: lightblue; }
  </style>`);
	});
	$$renderer.push(`<h1 class="svelte-xildax">hi</h1>`);
}
