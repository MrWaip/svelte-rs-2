import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-p6hspt",
	code: ".icon.svelte-p6hspt::before {content:\"\\ff\";}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<span class="icon svelte-p6hspt"></span>`);
}
