import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { onClose, onSubmit = onClose } = $$props;
	$$renderer.push(`<button>x</button>`);
}
