import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-354c5f",
	code: ".box.svelte-354c5f {color:red;width:10px;}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<div class="box svelte-354c5f">box</div>`);
}
