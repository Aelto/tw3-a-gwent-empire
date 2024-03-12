@context(
  note("gwintManager changes")
  define("mod.aGwentEmpire")
  file("game/gui/menus/gwintManager.ws")
  at(class CR4GwintManager)
)

@insert(
  note("set private variable to public")
  select[[
    private var diff1 : int;
    private var diff2 : int;
    private var diff3 : int;
    private var diff4 : int;
    private var diff5 : int;
    private var diff6 : int;
    private var diff7 : int;
    private var diff8 : int;
    private var diff9 : int;
    private var diff10 : int;
    private var diff11 : int;
    private var diff12 : int;
    private var diff13 : int;
    private var diff14 : int;
    private var diff15 : int;
  ]]
)
// modAGwentEmpire - BEGIN
public var diff1 : int;
public var diff2 : int;
public var diff3 : int;
public var diff4 : int;
public var diff5 : int;
public var diff6 : int;
public var diff7 : int;
public var diff8 : int;
public var diff9 : int;
public var diff10 : int;
public var diff11 : int;
public var diff12 : int;
public var diff13 : int;
public var diff14 : int;
public var diff15 : int;
// modAGwentEmpire - END

@insert(
  at(function SetEnemyDeckByName)
  above(switch(deckname))
)
// modGwentAddict - BEGIN
if (deckname == 'AGwentEmpireCustomDeck') {
  selectedEnemyDeck = GA_CONSTANTS_ENUM_CUSTOM_DECK_INDEX;

  return;
}
// modGwentAddict - END

@insert(
  at(function GetCurrentAIDeck)
  below(setupEnemyDecks())
)
// modGwentAddict - BEGIN
if (this.selectedEnemyDeck == GA_CONSTANTS_ENUM_CUSTOM_DECK_INDEX) {
  return GA_onCustomGwentStarted(this);
} 
// modGwentAddict - BEGIN