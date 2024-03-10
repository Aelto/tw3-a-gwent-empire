@context(
  define("mod.aGwentEmpire")
  file("game/player/r4Player.ws")
  at(class CR4Player)
)

@insert(
  at(function SetGwintMinigameState)
  below(gwintMinigameState = minigameState)
)
// modAGwentEmpire - BEGIN
GA_onGwentMatchEnded(minigameState);
// modAGwentEmpire - END