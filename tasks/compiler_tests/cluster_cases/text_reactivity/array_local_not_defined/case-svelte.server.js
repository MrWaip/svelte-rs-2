import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let log = [];
	function add() {
		log.push(1);
	}
	$$renderer.push(`<p>v ${$.escape(log)}</p> <button>+</button>`);
}
