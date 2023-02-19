
/**
 * the struct storing the general data for the mod. It is stored in the
 * GwintManager for easy & global access.
 */
struct GA_GeneralData {
  /**
   * The opponent for the next/ongoing match of gwent
   */
  var opponent: CNewNpc;

  var current_match: GA_GeneralDataCurrentMatch;
}

struct GA_GeneralDataCurrentMatch {
  var identifier: GA_DeckIdentifier;
}

/**
 * A number that is generated from the seed which should be in
 * the [0;inf[ range. It will use high numbers for good precision
 * when generating the difficulty, a common range is [0;10000[
 * where 0 is the easiest and 10000 is the hardest.
 */
struct GA_MatchDifficulty {
  var value: int;
}

/**
 * The available points for the deck.
 * This is mostly used when generating the deck to know how many
 * cards it can have. Each card has a cost and this controls how
 * much it can insert into the deck.
 */
struct GA_DeckPoints {
  var value: int;
}

struct GA_DeckIdentifier {
  var value: string;
}

struct GA_deckGenerationData {
  var identifier: GA_DeckIdentifier;
}