import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $s = () => $.store_get($.get(s), "$s", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const s = $.derived(() => $$props.store);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $s().keys, (key) => key, ($$anchor, key) => {
		const column = $.derived(() => $$props.columns[$.get(key)]);
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(column)));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
