import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { rest } = $$props;
	$$renderer.push(`<div${$.attributes({ ...rest })}></div>`);
}
