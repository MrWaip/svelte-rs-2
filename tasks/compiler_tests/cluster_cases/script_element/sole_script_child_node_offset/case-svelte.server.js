import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div>`);
	$$renderer.push(`<script><\/script>`);
	$$renderer.push(`<!----></div> ${$.html(x)}`);
}
