import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	{
		let dt = $.derived(() => [
			1,
			2,
			3
		]);
		$$renderer.push(`<div>${$.escape(dt.length)}
	${$.escape(dt.map((x) => x + dt().length))}</div>`);
	}
}
