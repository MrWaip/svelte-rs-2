import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = [1, 2], $$array = $.to_array(tmp, 2), x = $$array[0], y = $$array[1];
	$$renderer.push(`<p>${$.escape(x)} ${$.escape(y)}</p>`);
}
