import { rootNodeFromAnchorWithoutDefaultVisitor, AnchorIdl } from "@codama/nodes-from-anchor";
import { renderRustVisitor } from "@codama/renderers";
import { visit } from "@codama/visitors-core";
import idl from "./idl/solana_pinocchio_starter.json";

async function generateRustClient() {
  const node = rootNodeFromAnchorWithoutDefaultVisitor(idl as AnchorIdl);
  await visit(node, await renderRustVisitor("./src/generated-rust"));
  console.log("✅ Codama Rust client generated in client/generated-rust!");
}

generateRustClient();

