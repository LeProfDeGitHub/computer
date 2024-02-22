import { invoke } from "@tauri-apps/api/tauri";

const button   = document.getElementById("Invoker") as HTMLButtonElement;
const ram_text = document.getElementById("RAM")     as HTMLTextAreaElement;

async function greet() {
	ram_text.textContent = await invoke("greet", {});
}

button.addEventListener("click", greet)
