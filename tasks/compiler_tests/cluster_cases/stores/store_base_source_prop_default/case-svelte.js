import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $selectionWStore = () => $.store_get(selectionWStore(), "$selectionWStore", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let selectionWStore = $.prop($$props, "selectionWStore", 3, undefined);
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $selectionWStore()?.value));
	$.append($$anchor, div);
	$.pop();
	$$cleanup();
}
