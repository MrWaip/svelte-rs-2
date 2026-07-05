import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let props = {
		id: "a",
		class: "from-spread"
	};
	let active = true;
	$$renderer.push(`<div${$.attributes({ ...props }, void 0, { active })}></div>`);
}
