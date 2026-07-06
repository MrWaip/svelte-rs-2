import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div class="card svelte-1u8u4ji"><h2 class="title svelte-1u8u4ji">inside</h2></div> <section class="panel svelte-1u8u4ji"><h3 class="label svelte-1u8u4ji">implicit</h3></section> <h2 class="title">outside</h2> <h3 class="label">outside implicit</h3>`);
}
