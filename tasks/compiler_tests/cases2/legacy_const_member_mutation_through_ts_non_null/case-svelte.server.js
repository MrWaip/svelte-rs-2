import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function loader() {
		return { data: { selected: null } };
	}
	const state = loader();
	function reset() {
		if (state.data === null) return;
		state.data.selected = null;
	}
	function pick(value) {
		state.data.selected = value;
	}
	if (state.data) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<button>${$.escape(state.data.selected)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <button>x</button>`);
}
