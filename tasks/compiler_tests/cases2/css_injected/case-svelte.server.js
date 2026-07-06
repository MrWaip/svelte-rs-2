import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-1a7i8ec",
	code: "p.svelte-1a7i8ec {color:red;}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<p class="svelte-1a7i8ec">styled</p>`);
}
