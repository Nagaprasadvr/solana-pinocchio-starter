import { rootNodeFromAnchorWithoutDefaultVisitor, AnchorIdl } from "@codama/nodes-from-anchor";
import { renderJavaScriptVisitor, renderRustVisitor } from "@codama/renderers";
import { visit } from "@codama/visitors-core";
import idl from "./idl/solana_pinocchio_starter.json";

async function generateTypeScriptClient() {
  const node = rootNodeFromAnchorWithoutDefaultVisitor(idl as AnchorIdl);
  await visit(node, await renderJavaScriptVisitor("./ts/generated"));
  console.log("✅ Codama TypeScript client generated in client/ts/generated!");
}

async function generateRustClient() {
  const node = rootNodeFromAnchorWithoutDefaultVisitor(idl as AnchorIdl);
  await visit(node, await renderRustVisitor("./rust/generated"));
  console.log("✅ Codama Rust client generated in client/rust/generated!");
}

async function main() {
  const lang = process.argv[2];
  
  if (!lang) {
    console.log("Usage: tsx generate-clients.ts <typescript|rust|all>");
    process.exit(1);
  }

  try {
    if (lang === "typescript") {
      await generateTypeScriptClient();
    } else if (lang === "rust") {
      await generateRustClient();
    }
      else {
      console.log("Unknown language:", lang);
      process.exit(1);
    }
  } catch (error) {
    console.error("Generation failed:", error);
    process.exit(1);
  }
}

main();

