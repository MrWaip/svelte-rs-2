import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[13, 4]]);
var root_1 = $.add_locations($.from_html(`<button>update</button> <!>`, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $state = () => ($.validate_store($.get(state), "state"), $.store_get($.get(state), "$state", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let state = $.tag($.mutable_source("hello"), "state");
	function update() {
		$.store_unsub($.set(state, $.get(state) + "!"), "$state", $$stores);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			const len = $.tag($.derived_safe_equal(() => ($.get(state), $.untrack(() => $.get(state).length))), "len");
			$.get(len);
			var span = root();
			var text = $.child(span);
			$.reset(span);
			$.template_effect(() => $.set_text(text, `${$.get(len) ?? ""} / ${$state() ?? ""}`));
			$.append($$anchor, span);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(state)) $$render(consequent);
		}), "if", App, 11, 0);
	}
	$.event("click", button, update);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
