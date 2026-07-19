import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-limxtm",
	code: ".a.svelte-limxtm,\n	.b.svelte-limxtm {color:red;}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<div class="a svelte-limxtm">a</div> <div class="b svelte-limxtm">b</div>`);
}
