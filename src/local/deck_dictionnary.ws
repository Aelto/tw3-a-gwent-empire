
/**
 * An abstract class you should extend to form a dictionnary for the cards.
 *
 * The dictionnary's role is to populate the faction decks and their possible cards.
 * All this should be done with specific rules in order to get a generic way to access
 * easy cards for a given faction for example.
 */
abstract class GA_CardsDictionnary {

  /**
   * Since in the vanilla code factions are indexed using thousands like:
   * 1000, 2000, 3000, ... up to 5000 the indices here correspond to those
   * number divided by 1000.
   * 
   * If the dictionnary is meant to support 3rd party factions from other mods
   * this should be taken in account.
   */
  public var factions: array<GA_FactionCardDictionnary>;

  public function populate(difficulty: GA_MatchDifficulty) {
    // ...
    // add your generation code here.
    // Do whatever you want to generate more or less balanced decks
    // based on the received difficulty.
    // ...
  }
}

/**
 * A dictionnary of cards for a specific faction
 */
struct GA_FactionCardDictionnary {
  var hero_cards: array<GA_CardsDictionnaryEntry>;
  var leader_cards: array<GA_CardsDictionnaryEntry>;
  var unit_cards: array<GA_CardsDictionnaryEntry>;
}

/**
 * An dictionnary of cards for a specific role.
 *
 * It contains 
 */
struct GA_CardsDictionnaryEntry {
  var easy_cards: array<Ga_CardEntry>;
  var medium_cards: array<Ga_CardEntry>;
  var hard_cards: array<Ga_CardEntry>;
}

/**
 * A dictionnary entry for one specific card
 */
struct Ga_CardEntry {
  /**
   * The index for the card
   */
  var index: int;

  /**
   * The maximum amount of times this card will be added
   * to the deck if it is picked by the generator once.
   */
  var maximum: int;

  /**
   * The minimum amount of times this card will be added
   * to the deck if it is picked by the generator once.
   */
  var minimum: int;
}