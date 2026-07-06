import * as $ from "svelte/internal/server";
import A from "./A.svelte";
import B from "./B.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div class="island">`);
	A($$renderer, {});
	$$renderer.push(`<!---->  `);
	B($$renderer, {});
	$$renderer.push(`<!----></div>`);
}
