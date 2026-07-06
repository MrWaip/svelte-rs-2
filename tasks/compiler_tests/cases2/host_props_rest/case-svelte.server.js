import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { answer, $$slots, $$events, ...rest } = $$props;
	let host = void 0;
	$$renderer.push(`<p>${$.escape(answer)}</p>`);
}
