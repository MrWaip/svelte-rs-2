import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let point = {
		left: 1,
		right: 2
	};
	let tmp = point, left = tmp.left, right = tmp.right;
	function swap() {
		[left, right] = [right, left];
	}
	$$renderer.push(`<button>${$.escape(left)}:${$.escape(right)}</button>`);
}
