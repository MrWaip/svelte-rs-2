import * as $ from "svelte/internal/server";
import { i18n } from "somewhere";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div>${$.escape(i18n("a", "b"))}</div>`);
	});
}
