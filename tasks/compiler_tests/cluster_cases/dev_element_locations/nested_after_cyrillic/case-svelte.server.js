import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<section>Раздел <span>Внутри <em>текст</em></span></section>`);
}
