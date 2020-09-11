import { Orchestrator, Config } from "@holochain/tryorama";
import { cond } from "lodash";

const orchestrator = new Orchestrator();

export const simpleConfig = {
  alice: Config.dna("../multiplecalls.dna.gz", null),
  bobbo: Config.dna("../multiplecalls.dna.gz", null),
};

/* 
orchestrator.registerScenario("multiple gets", async (s, t) => {
  const { conductor } = await s.players({
    conductor: Config.gen(simpleConfig),
  });
  await conductor.spawn();

  let hash = await conductor.call('alice', "multiplecalls", "create_entry", 'hello world');
  let result = await conductor.call('alice', 'multiplecalls', 'get_entry_multiple', [hash, 15]);
  t.equal(result, 'hello world');

  hash = await conductor.call('alice', "multiplecalls", "create_entry", 'hello world');
  result = await conductor.call('alice', 'multiplecalls', 'get_entry_multiple', [hash, 130]);
  t.equal(result, 'hello world');
});

orchestrator.registerScenario("multiple commits", async (s, t) => {
  const { conductor } = await s.players({
    conductor: Config.gen(simpleConfig),
  });
  await conductor.spawn();

  await conductor.call('alice', "multiplecalls", "create_entry_multiple", ['hello world', 1]);
  t.ok(true, true);

  await conductor.call('alice', "multiplecalls", "create_entry_multiple", ['hello world', 140]);
  t.ok(true, true);
}); */

async function commitEntry(t, conductor, playerName, char) {
  const entry = `${char}`;
  const hash = await conductor.call(playerName, "multiplecalls", "create_entry", entry);
  t.ok(hash);

  const result = await conductor.call(playerName, 'multiplecalls', 'get_entry_multiple', [hash, 1]);
  t.ok(result);
}

orchestrator.registerScenario("many commits", async (s, t) => {
  const { conductor } = await s.players({
    conductor: Config.gen(simpleConfig),
  });
  await conductor.spawn();

  const NUMBER_OF_ENTRIES_TO_COMMIT = 300;
  for (let i = 0; i < NUMBER_OF_ENTRIES_TO_COMMIT; i++) {
    await commitEntry(t, conductor, 'alice', String.fromCharCode('a'.charCodeAt(0) + i));
  }

  const stateDump = await conductor.stateDump('alice');

  t.equal(stateDump.length - 4, NUMBER_OF_ENTRIES_TO_COMMIT)
});

orchestrator.run();
