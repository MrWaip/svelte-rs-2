import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-1ngs2ez",
	code: "\n	@media (min-width: 100px) {.box.svelte-1ngs2ez {color:red;}\n	}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<div class="box svelte-1ngs2ez">box</div>`);
}
