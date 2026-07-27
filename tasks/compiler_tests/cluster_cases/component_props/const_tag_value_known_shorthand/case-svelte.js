import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	const row = ($$anchor) => {
		const kLit = $.derived(() => "x");
		const kArith = $.derived(() => plain + 1);
		Child($$anchor, {
			kLit: $.get(kLit),
			kArith: $.get(kArith)
		});
	};
	let plain = 7;
	row($$anchor);
}
