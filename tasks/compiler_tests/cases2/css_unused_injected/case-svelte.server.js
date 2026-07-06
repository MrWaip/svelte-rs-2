import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-1n36she",
	code: ".used.svelte-1n36she {color:red;}.used.svelte-1n36she {border:1px solid;}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<div class="used svelte-1n36she">used</div>`);
}
