import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-472ibj",
	code: ".box {color:red;}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<div class="box">box</div>`);
}
