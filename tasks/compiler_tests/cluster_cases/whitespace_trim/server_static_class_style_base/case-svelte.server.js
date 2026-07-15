import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<span class="tsCompact300XSmall">x</span> <div style="--a: b;">y</div>`);
}
