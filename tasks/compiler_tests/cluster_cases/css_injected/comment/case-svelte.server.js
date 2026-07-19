import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-kpvy84",
	code: "\n	/* a comment */.box.svelte-kpvy84 {color:red;}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<div class="box svelte-kpvy84">box</div>`);
}
