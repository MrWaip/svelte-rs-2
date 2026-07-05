App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { el } from "./stores";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $el = () => ($.validate_store(el, "el"), $.store_get(el, "$el", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_this(div, ($$value) => $.store_set(el, $$value), () => $el());
	$.append($$anchor, div);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
