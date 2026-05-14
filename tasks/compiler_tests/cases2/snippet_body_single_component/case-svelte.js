import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	{
		const right = ($$anchor) => {
			Btn($$anchor, {});
		};
		Header($$anchor, {
			right,
			$$slots: { right: true }
		});
	}
}
