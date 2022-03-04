
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

struct GA_DeckIdentifier {
  var value: string;
}

struct GA_DeckPoints {
  var value: int;
}

struct GA_DeckSeed {
  var value: int;
}

struct GA_deckGenerationData {
  var identifier: GA_DeckIdentifier;
}