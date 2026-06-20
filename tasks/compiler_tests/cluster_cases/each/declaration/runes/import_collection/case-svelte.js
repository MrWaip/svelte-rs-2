import * as $ from "svelte/internal/client";
import { foo } from "./utils";
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => foo.bar, $.index, ($$anchor, bar) => {
		var span = root_1();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(bar)));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
	$.pop();
}
