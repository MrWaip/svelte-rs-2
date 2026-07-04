import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let boxes = [{ k1: "a" }];
	let area = "";
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => boxes, $.index, ($$anchor, box) => {
		const computed_const = $.tag($.derived_safe_equal(() => {
			const { i = 1, [`k${i}`]: sideone, [`k${area}${i + 1}`]: sidetwo } = $.get(box);
			return {
				i,
				sideone,
				sidetwo
			};
		}), "[@const]");
		$.get(computed_const);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(computed_const).sideone ?? ""}${$.get(computed_const).sidetwo ?? ""}${$.get(computed_const).i ?? ""}`));
		$.append($$anchor, button);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
