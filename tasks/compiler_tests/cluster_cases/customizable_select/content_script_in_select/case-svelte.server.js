import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<select>`);
	$$renderer.option({ value: "a" }, ($$renderer) => {
		$$renderer.push(`A`);
	});
	$$renderer.push(`<script>console.log('hi')<\/script>`);
	$$renderer.push(`<!></select>`);
}
