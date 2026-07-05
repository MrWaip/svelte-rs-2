import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let getKey = $.prop($$props, "getKey", 8);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 0, () => [
		1,
		2,
		3
	], (item) => getKey()(), ($$anchor, item) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, item));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
	$.pop();
}
