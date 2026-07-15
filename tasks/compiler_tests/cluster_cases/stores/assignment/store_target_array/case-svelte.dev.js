import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $u = () => ($.validate_store(u, "u"), $.store_get(u, "$u", $$stores));
	const $v = () => ($.validate_store(v, "v"), $.store_get(v, "$v", $$stores));
	const $w = () => ($.validate_store(w, "w"), $.store_get(w, "$w", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const u = writable(1);
	const v = writable(2);
	const w = writable(3);
	function run() {
		(($$value) => {
			var $$array = $.to_array($$value, 3);
			$.store_set(u, $$array[0]);
			$.store_set(v, $$array[1]);
			$.store_set(w, $$array[2]);
		})([
			7,
			8,
			9
		]);
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$u() ?? ""}${$v() ?? ""}${$w() ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
