import codegen from "@cosmwasm/ts-codegen";

codegen({
  contracts: [
    {
      name: "ShitStrap",
      dir: "../schema",
    },
    {
      name: "Cw20Base",
      dir: "../external-protos/cw20",
    },
    {
      name: "ShitStrapFactory",
      dir: "../external-protos/shitstrap-factory",
    },
  ],
  outPath: "./src/",

  // options are completely optional ;)
  options: {
    bundle: {
      bundleFile: "index.ts",
      scope: "contracts",
    },
    types: {
      enabled: true,
    },
    client: {
      enabled: true,
    },
    reactQuery: {
      enabled: true,
      optionalClient: false,
      version: 'v4',
      mutations: true,
      queryKeys: true,
      queryFactory: true,
    },
    recoil: {
      enabled: false,
    },
    messageComposer: {
      enabled: true,
    },
  },
}).then(() => {
  console.log("✨ all done!");
});
