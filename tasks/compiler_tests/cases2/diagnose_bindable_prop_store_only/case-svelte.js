import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button> <input/>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $inputValue = () => $.store_get($$props.inputValue, "$inputValue", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	function set5() {
		$$props.inputValue.set(5);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.child(button);
	$.reset(button);
	var input = $.sibling(button, 2);
	$.remove_input_defaults(input);
	$.template_effect(() => $.set_text(text, `read: ${$inputValue() ?? ""}`));
	$.delegated("click", button, set5);
	$.bind_value(input, $inputValue, ($$value) => $.store_set($$props.inputValue, $$value));
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
