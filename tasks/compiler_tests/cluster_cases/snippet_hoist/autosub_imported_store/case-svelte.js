import * as $ from "svelte/internal/client";
import { count } from "./store_mod.js";
export default function App($$anchor) {
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const foo = ($$anchor) => {
		$.next();
		var text = $.text();
		$.template_effect(() => $.set_text(text, $count()));
		$.append($$anchor, text);
	};
	foo($$anchor);
	$$cleanup();
}
