
enum GA_CONSTANTS_ENUM {
  GA_CONSTANTS_ENUM_CUSTOM_DECK_INDEX = 4304 // random number

  GA_CONSTANTS_ENUM_POINTS_FROM_SEED_ROLL = 10
}

state GwentAddictGlobalInteractionListener in SU_NpcInteraction_GlobalEventHandler extends GlobalEventListener {
  event OnEnterState(previous_state_name: name) {
    super.OnEnterState(previous_state_name);

    this.MyGlobalEventListener_main();
  }

  /**
   * states can have undefined behaviors when two states have methods with the
   * same name. For this reason it is recommend to use a prefix on your function
   * names, using the state name is a good solution.
   */
  entry function MyGlobalEventListener_main() {
    var gwintManager: CR4GwintManager;
    var receptor: CPeristentEntity;
    var actionName: string;
    var activator: CEntity;

    // now here you can do anything you want, here we print all the information
    // we have about the interaction.
    theGame
      .GetGuiManager()
      .ShowNotification(
        "received interaction: " + parent.actionName +
        ", activator: " + parent.activator.ToString() +
        ", receptor: " + parent.receptor.ToString()
      );

    gwintManager = theGame.GetGwintManager();
    gwintManager.testMatch = true;

    if (!gwintManager.GA_generalData) {
      gwintManager.GA_generalData = GA_GeneralData();
    }

    gwintManager.GA_generalData.opponent = (CNewNpc)parent.receptor;

    thePlayer.OnGwintGameRequested('GwentAddictCustomDeck', GwintFaction_NothernKingdom);

    // when our job is done, we MUST call the finish method. Not doing so will
    // block the global event listener and it won't work for the rest of the
    // session until the player gets a loading screen.
    this.finish();
  }
}