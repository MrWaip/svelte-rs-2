import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<button>update</button> <!>`, 1);
export default function App($$anchor) {
	const $state = () => $.store_get($.get(state), "$state", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let state = $.mutable_source("hello");
	function update() {
		$.store_unsub($.set(state, $.get(state) + "!"), "$state", $$stores);
	}
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			const len = $.derived_safe_equal(() => ($.get(state), $.untrack(() => $.get(state).length)));
			var span = root();
			var text = $.child(span);
			$.reset(span);
			$.template_effect(() => $.set_text(text, `${$.get(len) ?? ""} / ${$state() ?? ""}`));
			$.append($$anchor, span);
		};
		$.if(node, ($$render) => {
			if ($.get(state)) $$render(consequent);
		});
	}
	$.event("click", button, update);
	$.append($$anchor, fragment);
	$$cleanup();
}
