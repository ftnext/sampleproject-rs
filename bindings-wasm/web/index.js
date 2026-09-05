import init, { add_one } from "./pkg/bindings_wasm.js";

async function main() {
  await init();

  const result = add_one(20);
  console.log(result);

  document.querySelector("#result").textContent = String(result);
}

main();
