import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
import { BRAND } from "./brand";
export default function App($$renderer) {
	Comp($$renderer, { title: `prefix ${$.stringify(BRAND)}` });
}
