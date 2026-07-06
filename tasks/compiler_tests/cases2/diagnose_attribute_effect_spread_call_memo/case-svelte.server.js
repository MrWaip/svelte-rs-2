import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let classes = [];
	function mapClasses(base, ...rest) {
		return { [base]: true };
	}
	$$renderer.push(`<div${$.attributes({ ...mapClasses("base", ...classes) }, void 0, { active: true })}></div>`);
}
