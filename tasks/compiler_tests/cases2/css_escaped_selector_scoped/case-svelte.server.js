import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div class="foo:bar svelte-os1qct">class</div> <div id="hero:id" class="svelte-os1qct">id</div> <div class="miss">outside</div>`);
}
