import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/> <button>reset</button>`, 1);
export default function App($$anchor, $$props) {
	const $limitAmount = () => $.store_get($$props.limitAmount, "$limitAmount", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var button = $.sibling(input, 2);
	$.bind_value(input, $limitAmount, ($$value) => $.store_set($$props.limitAmount, $$value));
	$.delegated("click", button, () => $.store_set($$props.limitAmount, undefined));
	$.append($$anchor, fragment);
	$$cleanup();
}
$.delegate(["click"]);
