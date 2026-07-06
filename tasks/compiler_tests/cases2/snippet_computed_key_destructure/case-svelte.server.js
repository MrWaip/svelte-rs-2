import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let data = {
		label: "world",
		extra: "ok"
	};
	function key() {
		return "label";
	}
	function view($$renderer, { [key()]: value, ...rest }) {
		$$renderer.push(`<p>${$.escape(value)} ${$.escape(rest.extra)}</p>`);
	}
	view($$renderer, data);
}
