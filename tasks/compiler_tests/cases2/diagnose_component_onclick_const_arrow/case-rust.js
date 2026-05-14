import * as $ from "svelte/internal/client";
import Btn from "./Btn.svelte";
export default function App($$anchor) {
	const save = () => {};
	Btn($$anchor, { onclick: save });
}
