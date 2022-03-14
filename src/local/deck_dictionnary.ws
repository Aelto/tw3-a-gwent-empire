
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

  /**
   * the main place where the `factions` array is filled. This is where all the
   * data about the generating functions are placed into the dictionnary.
   */
  public function init() {

  }

  public function populate(difficulty: GA_MatchDifficulty, points: GA_DeckPoints): array<GA_CardEntry> {
    var deck: array<Ga_CardEntry>;

    // ...
    // add your generation code here.
    // Do whatever you want to generate more or less balanced decks
    // based on the received difficulty.
    // ...

    return deck;
  }

  /**
   * Return all the cards that are smaller than the given difficulty
   */
  public function getAvailableCardEntriesForDifficulty(cards: array<GA_CardEntry>, difficulty: GA_MatchDifficulty): array<GA_CardEntry> {
    var filtered_cards: array<GA_CardEntry>;
    var i: int;

    for (i = 0; i < cards.Size(); i += 1) {
      if (cards[i].required_difficulty > difficulty) {
        continue;
      }

      filtered_cards.PushBack(cards[i]);
    }

    return filtered_cards;
  }

  /**
   * Return all the cards whose cost is lower than the given deck points
   */
  public function getAvailableCardEntriesForDeckPoints(cards: array<GA_CardEntry>, points: GA_DeckPoints): array<GA_CardEntry> {
    var filtered_cards: array<GA_CardEntry>;
    var i: int;

    for (i = 0; i < cards.Size(); i += 1) {
      if (cards[i].cost > points) {
        continue;
      }

      filtered_cards.PushBack(cards[i]);
    }

    return filtered_cards;
  }
}

/**
 * A dictionnary of cards for a specific faction
 */
struct GA_FactionCardDictionnary {
  var hero_cards: array<GA_CardEntry>;
  var leader_cards: array<GA_CardEntry>;
  var unit_cards: array<GA_CardEntry>;
}

/**
 * A dictionnary entry for one specific card
 */
struct GA_CardEntry {
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

  /**
   * Defines how many points in the GA_DeckPoints adding
   * this card will cost.
   */
  var cost: GA_DeckPoints;

  /**
   * Defines after what difficulty the card should start
   * appearing in decks
   */
  var required_difficulty: GA_MatchDifficulty;
}