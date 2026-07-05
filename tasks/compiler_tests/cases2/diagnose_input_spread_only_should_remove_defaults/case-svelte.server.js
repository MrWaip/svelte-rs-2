import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { extra = {} } = $$props;
	$$renderer.push(`<input${$.attributes({ ...extra }, void 0, void 0, void 0, 4)}/>`);
}
