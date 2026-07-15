import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "x", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 0, () => [
		1,
		2,
		3
	], $.index, ($$anchor, n) => {
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${n ?? ""}${x() ?? ""}`));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
