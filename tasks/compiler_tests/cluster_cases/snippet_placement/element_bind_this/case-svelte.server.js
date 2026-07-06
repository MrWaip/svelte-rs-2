import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let thisBug = void 0;
	function Bug($$renderer) {
		$$renderer.push(`<!---->cool`);
	}
	$$renderer.push(`<form></form> ${$.escape(typeof thisBug)}`);
}
