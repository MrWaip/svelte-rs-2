import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<select>`);
	$$renderer.option({ value: "a" }, ($$renderer) => {
		$$renderer.push(`<b>A</b>`);
		$$renderer.push(`<script>console.log('hi')<\/script>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`</select>`);
}
