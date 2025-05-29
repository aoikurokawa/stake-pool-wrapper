const path = require("node:path");
const { rootNode } = require("@codama/nodes");
const { rootNodeFromAnchor } = require("@codama/nodes-from-anchor");
const { readJson } = require("@codama/renderers-core");
const { visit } = require("@codama/visitors-core");
const { renderVisitor } = require("@codama/renderers-vixen-parser");

const project = "program-parser"; // folder name
const idl = readJson(path.join(__dirname, "token_2022.json"));

// Use the appropriate node constructor based on your IDL type:
// const node = rootNodeFromAnchor(idl); // for Anchor
const node = rootNode(idl.program); // for non-Anchor

visit(
  node,
  renderVisitor(path.join(__dirname, "src", "generated"), {
    sdkName: "spl_stake_pool_sdk", // snake_case SDK name
    crateFolder: __dirname,
    formatCode: true,
  })
);