import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let boxes = [{ k1: "a" }];
	let area = "";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => boxes, $.index, ($$anchor, box) => {
		const computed_const = $.derived_safe_equal(() => {
			const { i = 1, [`k${i}`]: sideone, [`k${area}${i + 1}`]: sidetwo } = $.get(box);
			return {
				i,
				sideone,
				sidetwo
			};
		});
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(computed_const).sideone ?? ""}${$.get(computed_const).sidetwo ?? ""}${$.get(computed_const).i ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
