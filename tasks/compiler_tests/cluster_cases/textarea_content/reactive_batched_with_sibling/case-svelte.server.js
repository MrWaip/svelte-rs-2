import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let query = "";
	let name = "";
	function upd() {
		query = "a";
		name = "b";
	}
	$$renderer.push(`<button>x</button> <input${$.attr("value", name)}/> <textarea>`);
	const $$body = $.escape(query);
	if ($$body) {
		$$renderer.push(`${$$body}`);
	} else {}
	$$renderer.push(`</textarea>`);
}
