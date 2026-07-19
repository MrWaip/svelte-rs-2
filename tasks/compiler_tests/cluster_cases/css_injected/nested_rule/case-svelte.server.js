import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-444rfg",
	code: ".outer.svelte-444rfg {color:red;.inner:where(.svelte-444rfg) {color:blue;}}"
};
export default function App($$renderer) {
	$$renderer.global.css.add($$css);
	$$renderer.push(`<div class="outer svelte-444rfg"><span class="inner svelte-444rfg">inner</span></div>`);
}
