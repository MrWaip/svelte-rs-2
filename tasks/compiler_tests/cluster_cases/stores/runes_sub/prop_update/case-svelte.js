import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	const $store = () => $.store_get($$props.store, "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $store()));
	$.delegated("click", button, () => $.update_store($$props.store, $store()));
	$.append($$anchor, button);
	$$cleanup();
}
$.delegate(["click"]);
