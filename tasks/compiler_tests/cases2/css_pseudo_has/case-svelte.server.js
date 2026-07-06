import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div class="card svelte-mv1sf"><span class="inside svelte-mv1sf">inside</span></div> <span class="inside">outside</span>`);
}
