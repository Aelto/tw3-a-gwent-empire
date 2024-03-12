@context(
  note("npc changes")
  define("mod.aGwentEmpire")
  file("game/npc/npc.ws")
  at(class CNewNPC)
)

@insert(
  note("set up the variable that holds the NPC uuid")
  above(event OnInteraction)
)
// modAGwentEmpire - BEGIN
public saved var GA_deck_identifier: GA_DeckIdentifier;
// modAGwentEmpire - END


@insert(
  note("add a hook to the OnInteraction of the NPC")
  at(event OnInteraction)
  above(LogChannel)
)
// modAGwentEmpire - BEGIN
GA_npcOnInteraction(actionName, activator, this);
// modAGwentEmpire - END