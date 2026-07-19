import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-1aej1md",
	code: ".a.svelte-1aej1md {color:red;}.b.svelte-1aej1md {color:blue;}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<div class="a svelte-1aej1md">a</div> <div class="b svelte-1aej1md">b</div>`);
}
