import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const title = "world";
	$$renderer.push(`<!---->world <div><br/> world</div> <div>world</div>`);
}
