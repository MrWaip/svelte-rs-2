import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $u = () => ($.validate_store(u, "u"), $.store_get(u, "$u", $$stores));
	const $v = () => ($.validate_store(v, "v"), $.store_get(v, "$v", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const u = writable(1);
	const v = writable(2);
	let foo = $.tag($.mutable_source(), "foo");
	let arr = [1, 2];
	function run() {
		$.set(foo, ((arr) => {
			var $$array = $.to_array(arr, 2);
			$.store_set(u, $$array[0]);
			$.store_set(v, $$array[1]);
			return arr;
		})(arr));
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(foo) ?? ""}${$u() ?? ""}${$v() ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
