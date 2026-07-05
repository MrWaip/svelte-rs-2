import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const store = { state: { show: true } };
	const close = () => {
		store.state.show = false;
	};
	$$renderer.push(`<button>close</button>`);
}
