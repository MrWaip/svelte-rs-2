import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let inc;
	$.effect_root(() => {
		let count = $.state(0);
		let double = $.derived(() => $.get(count) * 2);
		inc = () => {
			$.update(count);
			console.log($.get(double));
		};
	});
}
