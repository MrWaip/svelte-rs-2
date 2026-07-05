import * as $ from "svelte/internal/client";
var root = $.from_html(` <div> </div>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let items = $.proxy([
		1,
		2,
		3
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, (item) => item, ($$anchor, item) => {
		$.next();
		var fragment_1 = root();
		var text = $.first_child(fragment_1);
		var div = $.sibling(text);
		var text_1 = $.child(div, true);
		$.reset(div);
		$.template_effect(($0) => {
			$.set_text(text, `${$0 ?? ""} `);
			$.set_text(text_1, item);
		}, [() => (() => {
			$.user_pre_effect(() => {
				items;
			});
		})()]);
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
	$.pop();
}
