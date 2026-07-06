import * as $ from "svelte/internal/server";
function item($$renderer, Modal) {
	$$renderer.push(`<p>${$.escape(Modal)}</p>`);
}
export default function Modal_1($$renderer, $$props) {
	let { children } = $$props;
	item($$renderer, "hi");
}
