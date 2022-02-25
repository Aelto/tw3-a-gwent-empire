
function GA_generateRandomDeck(manager: CR4GwintManager): SDeckDefinition {
  var deck: SDeckDefinition;

  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]

  deck.dynamicCardRequirements.PushBack(manager.diff5);
  deck.dynamicCards.PushBack(2); 
  deck.dynamicCardRequirements.PushBack(manager.diff7);
  deck.dynamicCards.PushBack(2); 
  deck.dynamicCardRequirements.PushBack(manager.diff9);
  deck.dynamicCards.PushBack(103); 
  deck.dynamicCardRequirements.PushBack(manager.diff11);
  deck.dynamicCards.PushBack(102); 
  deck.dynamicCardRequirements.PushBack(manager.diff11);
  deck.dynamicCards.PushBack(109); 
  deck.dynamicCardRequirements.PushBack(manager.diff14);
  deck.dynamicCards.PushBack(101); 
  deck.dynamicCardRequirements.PushBack(manager.diff14);
  deck.dynamicCards.PushBack(12);

  deck.specialCard = 100; 
  deck.leaderIndex = 1001;

  return deck;
}

/**
 * return the amount of points given for the supplied seed.
 * Points are used when generating a deck of cards, each card
 * has a cost and the points determine how many cards can be
 * in the decks.
 */
function GA_getDeckPointsFromSeed(seed: GA_DeckSeed): GA_DeckPoints {
  var rng: RandomNumberGenerator;

  rng = (new RandomNumberGenerator in thePlayer).setSeed(seed.value)
    .useSeed(true);

  return GA_DeckPoints((int)rng.nextRange(100, 10));
}

function GA_getSeedFromDeckIdentifier(identifier: GA_DeckIdentifier): GA_DeckSeed {
  
}

/**
 * Transform an identifier into a number.
 * An identifier is a string following the given syntax:
 * Numbers seperated by dashes (-), 
 */
function GA_identifierToInt(identifier: string): int {
  var segment: string;
  var sub: string;
  var output: int;

  segment = identifier;

  while (StrLen(segment) > 0) {
    sub = StrBeforeFirst(segment, "-");
    output += StringToInt(sub);

    // the +1 is there to exclude the "-"
    segment = StrMid(segment, StrLen(sub) + 1);
  };

  return output;
}