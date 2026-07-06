import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div${$.attributes({
		visible: true,
		title: `idx: ${$.stringify(idx)}`,
		test,
		i18n,
		positive: true,
		...props,
		id: "unique",
		...rest
	})}></div>`);
}
