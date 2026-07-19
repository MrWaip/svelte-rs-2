import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-bulewn",
	code: ".box.svelte-bulewn {--gap: 10px;color:red;}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<div class="box svelte-bulewn">box</div>`);
}
