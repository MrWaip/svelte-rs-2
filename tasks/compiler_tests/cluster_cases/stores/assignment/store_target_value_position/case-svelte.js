import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $u = () => $.store_get(u, "$u", $$stores);
	const $v = () => $.store_get(v, "$v", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const u = writable(1);
	const v = writable(2);
	let foo = $.mutable_source();
	let arr = [1, 2];
	function run() {
		$.set(foo, ((arr) => {
			var $$array = $.to_array(arr, 2);
			$.store_set(u, $$array[0]);
			$.store_set(v, $$array[1]);
			return arr;
		})(arr));
	}
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(foo) ?? ""}${$u() ?? ""}${$v() ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
