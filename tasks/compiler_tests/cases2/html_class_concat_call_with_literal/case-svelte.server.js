import * as $ from "svelte/internal/server";
import { getProductName } from "./helpers";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div${$.attr_class(`x0${$.stringify(getProductName())}`)}>hi</div>`);
	});
}
