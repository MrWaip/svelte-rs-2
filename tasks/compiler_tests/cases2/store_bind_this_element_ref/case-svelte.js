import * as $ from "svelte/internal/client";
import { el } from "./stores";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const $el = () => $.store_get(el, "$el", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var div = root();
	$.bind_this(div, ($$value) => $.store_set(el, $$value), () => $el());
	$.append($$anchor, div);
	$$cleanup();
}
