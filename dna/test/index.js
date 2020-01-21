/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require("path");

const {
  Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const { assingRole, createRole } = require("./utils");

const dnaPath = path.join(__dirname, "../dist/dna.dna.json");

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require("tape")),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly
  )
});

const dna = Config.dna(dnaPath, "scaffold-test");
const conductorConfig = Config.gen(
  { roles: dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    }
  }
);

orchestrator.registerScenario("create and assign role", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );

  const aliceAddress = alice.instance("roles").agentAddress;
  const bobAddress = bob.instance("roles").agentAddress;

  // Test initial bad actions
  const result = await createRole("editor")(bob);
  t.notOk(result.Ok);

  result = await assignRole("editor", bobAddress)(alice);
  t.notOk(result.Ok);

  // Test good case
  result = await createRole("editor")(alice);
  t.ok(result.Ok);

  await s.consistency();

  result = await assignRole("editor", aliceAddress)(bob);
  t.notOk(result.Ok);

  result = await assignRole("editor", bobAddress)(alice);
  t.ok(result.Ok);

  await s.consistency();
});

orchestrator.registerScenario("make a secondary admin", async (s, t) => {
  const { alice, bob, carol } = await s.players(
    { alice: conductorConfig, bob: conductorConfig, carol: conductorConfig },
    true
  );

  const bobAddress = bob.instance("roles").agentAddress;
  const carolAddress = carol.instance("roles").agentAddress;

  // Test good case
  result = await createRole("administrator")(alice);
  t.ok(result.Ok);

  await s.consistency();

  result = await assignRole("administrator", bobAddress)(alice);
  t.ok(result.Ok);

  await s.consistency();

  // TODO: receive all roles
  result = await createRole("editor")(bob);
  t.ok(result.Ok);

  await s.consistency();

  result = await assignRole("editor", carolAddress)(bob);
  t.ok(result.Ok);

  await s.consistency();
});

orchestrator.run();
