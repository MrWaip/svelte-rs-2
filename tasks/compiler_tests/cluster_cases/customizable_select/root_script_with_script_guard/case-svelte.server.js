import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div>`);
	$$renderer.push(`<script>console.log('hi')<\/script>`);
	$$renderer.push(`<!----></div>`);
}
