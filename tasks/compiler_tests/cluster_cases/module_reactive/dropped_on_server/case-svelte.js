import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
let num = 2;
let square;
$: square = num * num;
export default function App($$anchor) {}
