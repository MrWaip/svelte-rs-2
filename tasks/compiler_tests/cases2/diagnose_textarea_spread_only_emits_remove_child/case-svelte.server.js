import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { extra = {} } = $$props;
	$$renderer.push(`<textarea${$.attributes({ ...extra })}></textarea>`);
}
