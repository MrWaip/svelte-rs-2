import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	const row = ($$anchor) => {
		let doubled;
		var promises_1 = $.run([() => promises[0].promise, () => doubled = $.derived(() => $.get(number) * 2)]);
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(doubled)), void 0, void 0, [promises_1[1]]);
		$.append($$anchor, span);
	};
	let n = 1;
	let number;
	var promises = $.run([async () => number = await $.async_derived(() => Promise.resolve(n))]);
	row($$anchor);
}
