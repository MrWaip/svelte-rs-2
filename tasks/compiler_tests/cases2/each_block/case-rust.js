import * as $ from "svelte/internal/client";
var root_1 = $.template(` <div> </div>`, 1);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => [
		1,
		2,
		3
	], $.index, ($$anchor, item) => {
		$.next();
		var fragment_1 = root_1();
		var text = $.first_child(fragment_1);
		var div = $.sibling(text);
		var text_1 = $.child(div);
		$.reset(div);
		$.template_effect(() => {
			$.set_text(text, `${item ?? ""} `);
			$.set_text(text_1, `${item ?? ""} + example`);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
